use super::discord_types::{
    InteractionCallbackType, InteractionRequest, InteractionResponse, InteractionType,
};
use super::user_types::{ApplicationCommand, MessageComponent, ModalSubmit, Response};

pub trait InteractionHandler {
    #[allow(unused)]
    fn handle_application_command(&self, ac: ApplicationCommand) -> Response {
        todo!();
    }

    #[allow(unused)]
    fn handle_message_component(&self, mc: MessageComponent) -> Response {
        todo!();
    }

    #[allow(unused)]
    fn handle_modal_submit(&self, ms: ModalSubmit) -> Response {
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

        InteractionType::ApplicationCommand => match handler.handle_application_command(req.into())
        {
            Response::Message(m) => m.into(),
            Response::Modal(m) => m.into(),
        },

        InteractionType::MessageComponent => match handler.handle_message_component(req.into()) {
            Response::Message(m) => m.into(),
            Response::Modal(m) => m.into(),
        },

        InteractionType::ModalSubmit => match handler.handle_modal_submit(req.into()) {
            Response::Message(m) => m.into(),
            Response::Modal(_) => panic!("Modal cannot result in another modal!"),
        },
    }
}
