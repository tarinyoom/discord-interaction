use discord_interaction::{
    run_handler, ApplicationCommand, ApplicationCommandResponse, Button, InteractionHandler,
};
use lambda_http::Error;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = DemoHandler {};
    run_handler(APPLICATION_PUBLIC_KEY, &handler).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_application_command(&self, ac: ApplicationCommand) -> ApplicationCommandResponse {
        match ac.command_name.as_str() {
            "hello" => ApplicationCommandResponse {
                text: format!("Hello <@{}>!", ac.user_id),
                buttons: vec![
                    Button {
                        text: "hello!".to_string(),
                        id: "hello/button1".to_string(),
                    },
                    Button {
                        text: "world.".to_string(),
                        id: "hello/button2".to_string(),
                    },
                ],
            },

            _ => todo!(),
        }
    }
}
