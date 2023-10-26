use super::discord_types::{
    InteractionCallbackType, InteractionRequest, InteractionResponse, InteractionType,
};
use super::user_types::{ApplicationCommand, MessageComponent, MessageResponse, ModalSubmit};

pub trait InteractionHandler {
    #[allow(unused)]
    fn handle_application_command(&self, ac: ApplicationCommand) -> MessageResponse {
        todo!();
    }

    #[allow(unused)]
    fn handle_message_component(&self, mc: MessageComponent) -> MessageResponse {
        todo!();
    }

    #[allow(unused)]
    fn handle_modal_submit(&self, ms: ModalSubmit) -> MessageResponse {
        todo!();
    }
}

pub fn handle_interaction<T>(handler: &T, req: &InteractionRequest) -> InteractionResponse
where
    T: InteractionHandler,
{
    match req.r#type {
        InteractionType::Ping => InteractionResponse {
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
