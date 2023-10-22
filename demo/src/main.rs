use lambda_http::Error;
use discord_interaction::{run_handler, InteractionHandler};

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = InteractionHandler{};
    run_handler(APPLICATION_PUBLIC_KEY, &handler).await
}
