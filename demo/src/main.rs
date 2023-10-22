use discord_interaction::{run_handler, InteractionHandler, InteractionType, Request, Response};
use lambda_http::Error;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = DemoHandler {};
    run_handler(APPLICATION_PUBLIC_KEY, &handler).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_interaction(&self, req: &Request) -> Response {
        match &req.r#type {
            InteractionType::Ping => Response::pong(),
            _ => Response::message().content("hello world").into(),
        }
    }
}
