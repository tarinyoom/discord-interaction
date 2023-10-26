use discord_interaction::{
    run_handler, ApplicationCommand, InteractionHandler, Message, MessageComponent, Modal,
    ModalSubmit, Response,
};
use lambda_http::Error;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_handler::<DemoHandler>(APPLICATION_PUBLIC_KEY).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        match ac.command_name.as_str() {
            "hello" => Response::Message(
                Message::new()
                    .text(&format!("Hello <@{}>!", ac.user_id))
                    .button("new_ephemeral", "spawn quiet message")
                    .button("edit", "change this message")
                    .button("modal", "try a modal"),
            ),

            _ => panic!(),
        }
    }

    fn handle_message_component(mc: MessageComponent) -> Response {
        match mc.id.as_str() {
            "new_ephemeral" => Response::Message(
                Message::new()
                    .text("You've spawned a new ephemeral message!")
                    .ephemeral(),
            ),

            "edit" => Response::Message(
                Message::new()
                    .text("You edited the existing message.")
                    .edit(),
            ),

            "modal" => Response::Modal(
                Modal::new()
                    .id("my_modal")
                    .title("Provide input values.")
                    .field("v1", "A value")
                    .field("v2", "Another value"),
            ),

            _ => panic!(),
        }
    }

    fn handle_modal_submit(ms: ModalSubmit) -> Response {
        match ms.id.as_str() {
            "my_modal" => {
                let v1 = ms.values.get("v1").unwrap();
                let v2 = ms.values.get("v2").unwrap();
                let text = format!("You entered the values `{}` and `{}`.", v1, v2);

                Response::Message(Message::new().text(&text))
            }

            _ => panic!(),
        }
    }
}
