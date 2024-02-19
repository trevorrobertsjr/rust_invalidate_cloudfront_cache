use tracing_subscriber::filter::{EnvFilter, LevelFilter};use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;use lambda_runtime::{run, service_fn, Error, LambdaEvent};


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-lambda/src/main.rs
/// - https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-lambda-external-runtime/src/main.rs
/// - https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-sdk/src/main.rs
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    // Extract some useful information from the request

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
