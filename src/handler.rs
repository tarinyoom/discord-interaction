use super::discord_types::{InteractionCallbackType, InteractionType, Request, Response};
use super::user_types::{
    ApplicationCommand, ApplicationCommandResponse, MessageComponent, MessageComponentResponse,
    ModalSubmit, ModalSubmitResponse,
};

pub trait InteractionHandler {
    fn handle_application_command(&self, ac: ApplicationCommand) -> ApplicationCommandResponse {
        todo!();
    }

    fn handle_message_component(&self, mc: MessageComponent) -> MessageComponentResponse {
        todo!();
    }

    fn handle_modal_submit(&self, ms: ModalSubmit) -> ModalSubmitResponse {
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
