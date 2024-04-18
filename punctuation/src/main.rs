use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Input {
    data: String,
}

#[derive(Deserialize, Serialize)]
struct Output {
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(process_data); 
    lambda_runtime::run(func).await?; 
    Ok(()) 
}

async fn process_data(event: Input, _ctx: Context) -> Result<Output, Error> {
    let mut result = String::new();
    
    for c in event.data.chars() {
        if c.is_alphanumeric() || c.is_whitespace() {
            result.push(c);
        }
    }
    
    Ok(Output { result })
}
