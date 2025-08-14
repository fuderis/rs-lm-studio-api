use crate::prelude::*;
use super::*;

use reqwest::Client;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// The LM Studio chat
pub struct Chat {
    pub(crate) model: Model,
    pub(crate) context: Context,
    pub(crate) host: String,
    pub(crate) client: Client,
    pub(crate) reader: Option<ResponseReader>,
}

impl Chat {
    /// Creates a new simple chat
    pub async fn new<M: Into<Model>, C: Into<Context>>(model: M, context: C, port: u16) -> Result<Self> {
        let mut chat = Self {
            model: model.into(),
            context: context.into(),
            host: fmt!("http://localhost:{port}"),
            client: Client::new(),
            reader: None,
        };

        // loading model:
        chat.load_model(chat.model.clone()).await?;

        Ok(chat)
    }

    /// Loads AI model to memory
    pub async fn load_model<M: Into<Model>>(&mut self, model: M) -> Result<()> {
        let request = Prompt {
            model: model.into(),
            prompt: str!("Hello"),
            stream: false,
            ..Default::default()
        }.into();

        let _ = self.send(request).await?;

        Ok(())
    }

    /// Send request to chat
    pub async fn send(&mut self, request: Request) -> Result<Option<Response>> {
        self.context.update_system_info().await;
        
        match request {
            Request::Messages(request) => self.handle_messages(request).await,
            Request::Prompt(request) => self.handle_prompt(request).await,
        }
    }

    /// Handle messages request
    async fn handle_messages(&mut self, mut request: Messages) -> Result<Option<Response>> {
        let url = fmt!("{}/v1/chat/completions", self.host);

        // choose AI model:
        if let Model::Other(s) = &request.model {
            if s.is_empty() {
                request.model = self.model.clone();
            }
        }
        
        // add request to context:
        request.messages = if request.context {
            for msg in request.messages {
                self.context.add(msg);
            }

            self.context.get()
        } else {
            let mut context = self.context.clone();
            for msg in request.messages {
                context.add(msg);
            }

            context.get()
        };

        // handle request:
        if !request.stream {
            let mut response = self.client.post(&url)
                .json(&request)
                .send()
                .await?
                .error_for_status()?
                .json::<Response>()
                .await?;

            // filtering <think>..</think> block:
            if request.skip_think {
                let re = re!(r"(?s)<think>.*?</think>");
                
                for choice in &mut response.choices {
                    let message = choice.message.as_mut().unwrap();
                    message.content = re.replace_all(&message.content, "").trim().to_string();
                }
            }

            // add response to context:
            if request.context {
                if let Some(choice) = response.choices.get(0) {
                    let message = choice.message.as_ref().unwrap();
                    let answer = Message::new(Role::Assistant, message.content.clone());
                    self.context.add(answer);
                }
            }

            Ok(Some(response))
        } else {
            // spawning stream reader:
            self.spawn_reader(url.clone(), request.clone(), request.context, request.skip_think).await?;

            Ok(None)
        }
    }

    /// Handle prompt request (without any context)
    async fn handle_prompt(&mut self, mut request: Prompt) -> Result<Option<Response>> {
        let url = fmt!("{}/v1/completions", self.host);

        // choose AI model:
        if let Model::Other(s) = &request.model {
            if s.is_empty() {
                request.model = self.model.clone();
            }
        }
        
        // add request to context:
        request.prompt = if request.context {
            self.context.add(request.prompt);
            self.context.get_as_string()
        } else {
            let mut context = self.context.clone();
            context.add(request.prompt);
            context.get_as_string()
        };

        // handle request:
        if !request.stream {
            let mut response = self.client.post(&url)
                .json(&request)
                .send()
                .await?
                .error_for_status()?
                .json::<Response>()
                .await?;

            if request.skip_think {
                let re = re!(r"(?s)<think>.*?</think>");
                
                for choice in &mut response.choices {
                    let text = choice.text.as_mut().unwrap();
                    *text = re.replace_all(&text, "").trim().to_string();
                }
            }

            Ok(Some(response))
        } else {
            // spawning stream reader:
            self.spawn_reader(url.clone(), request.clone(), false, request.skip_think).await?;

            Ok(None)
        }
    }

    /// Spawns stream reader
    async fn spawn_reader<J>(&mut self, url: String, request: J, context: bool, skip_think: bool) -> Result<()>
    where J: Serialize + Send + Sync + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel::<Result<StreamChoice>>();
        let client = self.client.clone();

        self.reader = Some( ResponseReader::new(UnboundedReceiverStream::new(rx), context) );
        
        tokio::spawn(async move {
            let mut is_thinking = false;
            let mut is_after_thinking = false;
            
            let response = client.post(&url)
                .json(&request)
                .send()
                .await;

            match response {
                Ok(response) => {
                    let mut stream = response.bytes_stream();

                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(chunk) => {
                                let chunk = String::from_utf8_lossy(&chunk);

                                for line in chunk.lines() {
                                    // parsing response line:
                                    if line.starts_with("data: ") {
                                        let data = &line[6..];
                                        if data == "[DONE]" {
                                            break;
                                        }

                                        let stream: Result<Stream> = json::from_str(data).map_err(Into::into);
                                        let stream = if let Ok(r) = stream { r }else{ continue };

                                        for mut choice in stream.choices {
                                            if let Some(text) = choice.text_mut() {
                                                // filtering <think>..</think> block:
                                                if skip_think {
                                                    if is_thinking {
                                                        if text.contains("</think>") {
                                                            is_thinking = false;
                                                            is_after_thinking = true;
                                                        }
                                                        continue;
                                                    }
                                                    else if text.contains("<think>") {
                                                        is_thinking = true;
                                                        continue;
                                                    }

                                                    // trim extra spaces:
                                                    if is_after_thinking {
                                                        *text = text.trim_start().to_string();
                                                        is_after_thinking = false;
                                                    }
                                                }
                                                
                                                // send answer part to channel:
                                                if tx.send(Ok(choice)).is_err() {
                                                    break;
                                                }
                                            } else {
                                                continue;
                                            }
                                        }
                                    }
                                }
                            },

                            Err(e) => {
                                let _ = tx.send(Err(e.into()));
                                break;
                            }
                        }
                    }
                },
                
                Err(e) => {
                    let _ = tx.send(Err(e.into()));
                }
            }
        });

        Ok(())
    }

    /// Read next stream choice
    pub async fn next(&mut self) -> Option<Result<StreamChoice>> {
        if let Some(reader) = &mut self.reader {
            let result = reader.next().await;

            if reader.context && reader.is_ready {
                self.context.add(reader.message.clone())
            }
            
            result
        } else {
            self.reader = None;
            None
        }
    }
}
