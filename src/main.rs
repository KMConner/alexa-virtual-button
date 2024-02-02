mod smart_home;

use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    Ok(json!({"message": "foobar"}))
}
