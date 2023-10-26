use discord_interaction::{
    run_handler, ApplicationCommand, Button, InteractionHandler, MessageComponent, MessageResponse,
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
    fn handle_application_command(&self, ac: ApplicationCommand) -> MessageResponse {
        match ac.command_name.as_str() {
            "hello" => MessageResponse::new()
                .text(&format!("Hello <@{}>!", ac.user_id))
                .button("hello!", "hello/button1")
                .button("world", "hello/button2"),

            _ => panic!(),
        }
    }

    fn handle_message_component(&self, mc: MessageComponent) -> MessageResponse {
        match mc.id.as_str() {
            "hello/button1" => MessageResponse::new()
                .text("You pressed the hello button!")
                .button("good bye", "hello/goodbye")
                .ephemeral(),

            "hello/button2" => MessageResponse::new()
                .text("You pressed the world button.")
                .edit(),

            "hello/goodbye" => MessageResponse::new().text("Good bye."),

            _ => panic!(),
        }
    }
}
