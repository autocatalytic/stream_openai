use async_openai::{types::CreateCompletionRequestArgs, Client};
use futures::StreamExt;
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let api_key = "INSERT OPENAI TOKEN";

    let client = Client::new().with_api_key(api_key);

    loop {
        println!("\n\n--- Enter prompt ---");
        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Failed to read line");

        let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .n(1)
            .prompt(&prompt)
            // .add_context("i am a professional engineer") // not yet implemented api call
            .stream(true)
            .max_tokens(1024_u16)
            .temperature(0.5)
            .build()?;

        let mut stream = client.completions().create_stream(request).await?;

        while let Some(response) = stream.next().await {
            match response {
                Ok(ccr) => ccr.choices.iter().for_each(|c| {
                    print!("{}", c.text);
                }),
                Err(e) => eprintln!("{}", e), 
            } 
        } 

    } 

    //Ok(())        // apparently this is unreachable
    
}
