use super::discord_types::{InteractionCallbackType, InteractionType, Request, Response};
use super::user_types::{ApplicationCommand, MessageComponent, MessageResponse, ModalSubmit};

pub trait InteractionHandler {
    fn handle_application_command(&self, ac: ApplicationCommand) -> MessageResponse {
        todo!();
    }

    fn handle_message_component(&self, mc: MessageComponent) -> MessageResponse {
        todo!();
    }

    fn handle_modal_submit(&self, ms: ModalSubmit) -> MessageResponse {
        todo!();
    }
}

pub fn handle_interaction<T>(handler: &T, req: &Request) -> Response
where
    T: InteractionHandler,
{
    match req.r#type {
        InteractionType::Ping => Response {
            r#type: InteractionCallbackType::Pong,
            data: None,
        },

        InteractionType::ApplicationCommand => {
            handler.handle_application_command(req.into()).into()
        }

        InteractionType::MessageComponent => handler.handle_message_component(req.into()).into(),

        InteractionType::ModalSubmit => handler.handle_modal_submit(req.into()).into(),
    }
}
