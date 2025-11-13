[![github]](https://github.com/fuderis/rs-lm-studio-api)&ensp;
[![crates-io]](https://crates.io/crates/lm-studio-api)&ensp;
[![docs-rs]](https://docs.rs/lm-studio-api)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# LM Studio API (UNOFFICIAL)

Is a high-performance and user-friendly library for interacting with locally running Llama-based language models via LM Studio. It allows you to send requests to models, receive responses both in full and in streaming mode, and manage model parameters.


## Key features:

* Support for both regular and streaming response modes.
* Context management and system prompt customization.
* Flexible configuration of request and model parameters.
* Supports structured response schemes in JSON format.


## Examples:

```rust
use lm_studio_api::prelude::*;

/// The system prompt
struct SystemPrompt;

impl SystemInfo for SystemPrompt {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
    
    fn update(&mut self) -> String {
        format!(r##"
            You're Jarvis — is a personal assistant created by the best programmer 'Fuderis'.

            Response briefly and clearly.
            Response language: English.
            
            Actual system Info:
            * datetime: 1969-10-29 22:30:00.
            * location: Russian Federation, Moscow.
        "##)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // init chat:
    let mut chat = Chat::new(
        Model::Gemma3_4b,                         // AI model
        Context::new(SystemPrompt::new(), 8192),  // system prompt + max tokens
        9090,                                     // server port
    );

    // generating request:
    let request = Messages {
        messages: vec![
            Message {
                role: Role::User,
                content: vec![
                    Content::Text { text: "What is shown in the picture?".into() },
                    Content::Image { image: Image::from_file("rust-logo.png").unwrap() }
                ]
            }
        ],
        context: true,
        stream: true,
        /* format: Some(Format::json(
            "commands",
            vec![
                Schema::object(
                    "datetime",
                    "returns actual datetime",
                    macron::hash_map! {
                        "time": Schema::string("only time", Some("time")),
                        "date": Schema::string("only date", Some("date")),
                    }
                ),

                Schema::object(
                    "location",
                    "returns user geolocation",
                    macron::hash_map! {
                        "location": Schema::string("user geolocation", None),
                    }
                ),
            ],
            false
        )), */
        ..Default::default()
    };
    
    // sending request:
    let _ = chat.send(request.into()).await?;

    // reading pre-results:
    while let Some(result) = chat.next().await {
        match result {
            Ok(r) => if let Some(text) = r.text() { eprint!("{text}"); }else{ },
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    Ok(())
}
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can [find me here](https://t.me/fuderis), also [see my channel](https://t.me/fuderis_club).
I welcome your suggestions and feedback!

> Copyright (c) 2025 *Bulat Sh.* ([fuderis](https://t.me/fuderis))
