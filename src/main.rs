extern crate lm_studio_api;  use lm_studio_api::{ prelude::*, Chat, Completions, Context, Model };

#[tokio::main]
async fn main() -> Result<()> {
    println!("Loading chat models..");

    // init chat:
    let mut chat = Chat::new(
        Model::Gemma3_4b,
        Context::new(
            "
                You're Jarvis - my personal assistant. Call me master.

                Answer me briefly and clearly.
                Output language: Russian.
            ".trim(),           // system prompt
            4090 // context size (in tokens)
        ),
        9090,  // LM Studio IP port
    ).await?;
    
    /* // NO STREAM TEST:
    
    println!(">> Hi, what's your name?");
    
    // init request:
    let request = Request {
        messages: vec!["Hi, what's your name?".into()],
        context: true,
        stream: false,
        ..Request::default()
    };

    // sending request:
    let result = chat.send(request).await;

    match result {
        Ok(Some(response)) => println!("<< {}", response.text()),
        Err(e) => eprintln!("Error: {e}"),
        _ => {}
    } */

    // STREAM TEST:

    println!("Chat ready, type message:");

    loop {
        // reading user input:
        println!("");
        eprint!(">> ");
        
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        eprint!("<< ");

        // init request:
        let request = Completions {
            messages: vec![buf.into()],
            context: true,
            stream: true,
            ..Completions::default()
        };
        
        // sending request:
        let _ = chat.send(request.into()).await.unwrap();

        // reading AI results:
        while let Some(result) = chat.next().await {
            match result {
                Ok(text) if !text.is_empty() => eprint!("{text}"),
                Err(e) => eprintln!("Error: {e}"),
                _ => {}
            }
        }
    }
}
