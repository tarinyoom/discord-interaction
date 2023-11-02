use discord_interaction::{
    run, ApplicationCommand, InteractionHandler, Message, MessageComponent, Modal, ModalSubmit,
    Response,
};
use lambda_http::Error;
use regex::Regex;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run::<DemoHandler>(APPLICATION_PUBLIC_KEY).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        match ac.command_name.as_str() {
            "hello" => Response::Message(
                Message::new()
                    .text(&format!("Hello <@{}>!", ac.user_id))
                    .button("the_button", "the button")
                    .button("modal", "input some text")
                    .button("spawn", "spawn new message"),
            ),

            _ => panic!(),
        }
    }

    fn handle_message_component(mc: MessageComponent) -> Response {
        match mc.id.as_str() {
            "the_button" => {
                let n = get_button_clicks(&mc.source.text).unwrap_or(0);
                Response::Message(
                    Message::new()
                        .text(&format!("You've clicked the button {} times.", n + 1))
                        .button("the_button", "the button")
                        .button("modal", "input some text")
                        .button("spawn", "spawn new message")
                        .edit(),
                )
            },

            "spawn" => Response::Message(
                Message::new()
                    .text("This is a new message. The message is also *ephemeral*, meaning it's only visible to you.")
                    .ephemeral(),
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
                let text = format!(
                    "{}\nYou entered the values `{}` and `{}`.",
                    ms.source.text, v1, v2
                );

                Response::Message(
                    Message::new()
                        .text(&text)
                        .button("the_button", "the button")
                        .button("modal", "input some text")
                        .button("spawn", "spawn new message")
                        .edit(),
                )
            }

            _ => panic!(),
        }
    }
}

fn get_button_clicks(msg: &str) -> Option<u64> {
    let pattern = "You've clicked the button [0-9]* times.";
    let re = Regex::new(&pattern).unwrap();
    let mut range = re.find(msg)?.range();
    range.start += 26;
    range.end -= 7;
    let n = msg[range].parse::<u64>().ok()?;
    Some(n)
}
