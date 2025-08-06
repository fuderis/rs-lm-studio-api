use crate::prelude::*;
use super::*;

use reqwest::Client;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// The LM Studio chat
pub struct Chat {
    model: Model,
    context: Context,
    host: String,
    client: Client,
    reader: Option<ResponseReader>,
}

impl Chat {   
    /// Creates a new chat
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
    async fn load_model<M: Into<Model>>(&mut self, model: M) -> Result<()> {
        let request = Messages {
            model: model.into(),
            messages: vec!["Hello".into()],
            context: false,
            stream: false,
            ..Messages::default()
        }.into();

        let _ = self.send(request).await?;

        Ok(())
    }

    /// Send request to chat
    pub async fn send(&mut self, request: Request) -> Result<Option<Response>> {
        match request {
            Request::Messages(request) => self.handle_messages(request).await,
            Request::Prompt(request) => self.handle_prompt(request).await,
        }
    }

    /// Handle messages request
    async fn handle_messages(&mut self, mut request: Messages) -> Result<Option<Response>> {
        let url = fmt!("{}/v1/chat/completions", self.host);
        
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

        // choose AI model:
        if let Model::Custom(s) = &request.model {
            if s.is_empty() {
                request.model = self.model.clone();
            }
        }

        // send simple request:
        if !request.stream {
            let mut response = self.client.post(&url)
                .json(&request)
                .send()
                .await?
                .error_for_status()?
                .json::<Response>()
                .await?;

            // filtering <think>..</think> block:
            if !request.think {
                let re = re!(r"(?s)<think>.*?</think>");
                
                for choice in &mut response.choices {
                    choice.message.content = re.replace_all(&choice.message.content, "").trim().to_string();
                }
            }

            // add response to context:
            if request.context {
                if let Some(choice) = response.choices.get(0) {
                    let answer = Message::new(Role::Assistant, choice.message.content.clone());
                    self.context.add(answer);
                }
            }

            Ok(Some(response))
        }
        // send request as stream:
        else
        {
            // init stream channel:
            let (tx, rx) = mpsc::unbounded_channel::<Result<String>>();
            let client = self.client.clone();
            let url = url.clone();
            let request_clone = request.clone();

            // running async task:
            tokio::spawn(async move {
                let mut is_thinking = false;
                let mut is_after_thinking = false;
                
                let response = client.post(&url)
                    .json(&request_clone)
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

                                            for StreamChoice { delta } in stream.choices {
                                                if let Delta { content: Some(mut part) } = delta {
                                                    // filtering <think>..</think> block:
                                                    if !request.think {
                                                        if is_thinking {
                                                            if part.contains("</think>") {
                                                                is_thinking = false;
                                                                is_after_thinking = true;
                                                            }
                                                            continue;
                                                        }
                                                        else if part.contains("<think>") {
                                                            is_thinking = true;
                                                            continue;
                                                        }

                                                        // trim extra spaces:
                                                        if is_after_thinking {
                                                            part = part.trim_start().to_string();
                                                            is_after_thinking = false;
                                                        }
                                                    }
                                                    
                                                    // send answer part to channel:
                                                    if tx.send(Ok(part)).is_err() {
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

            self.reader = Some( ResponseReader::new(UnboundedReceiverStream::new(rx), request.context) );

            Ok(None)
        }
    }

    /// Handle prompt request
    async fn handle_prompt(&mut self, mut request: Prompt) -> Result<Option<Response>> {
        let url = fmt!("{}/v1/completions", self.host);

        todo!();
    }

    /// Read next stream choice
    pub async fn next(&mut self) -> Option<Result<String>> {
        if let Some(reader) = &mut self.reader {
            let result = reader.next().await;

            if reader.context && reader.is_ready {
                self.context.add(reader.message.clone())
            }
            
            result
        } else {
            None
        }
    }
}
