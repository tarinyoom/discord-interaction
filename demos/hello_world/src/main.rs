use discord_interaction::{run, ApplicationCommand, InteractionHandler, Message, Response};
use lambda_http::Error;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run::<DemoHandler>(APPLICATION_PUBLIC_KEY).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        let text_content = format!("Hello <@{}>!", ac.user_id);
        Response::Message(Message::new().text(&text_content))
    }
}
