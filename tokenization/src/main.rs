// Import necessary modules
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Input {
    result: String, // Input text to be processed (remove punc)
}

#[derive(Deserialize, Serialize)]
struct Output {
    tokens: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(process_data); 
    lambda_runtime::run(func).await?; 
    Ok(()) 
}

async fn process_data(event: Input, _ctx: Context) -> Result<Output, Error> {
    let tokens: Vec<String> = event.result.split_whitespace().map(|s| s.to_string()).collect();
    Ok(Output { tokens }) 
}
