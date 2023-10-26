use super::discord_types::{
    InteractionCallbackType, InteractionRequest, InteractionResponse, InteractionType,
};
use super::user_types::{ApplicationCommand, MessageComponent, ModalSubmit, Response};

pub trait InteractionHandler {
    #[allow(unused)]
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        todo!();
    }

    #[allow(unused)]
    fn handle_message_component(mc: MessageComponent) -> Response {
        todo!();
    }

    #[allow(unused)]
    fn handle_modal_submit(ms: ModalSubmit) -> Response {
        todo!();
    }
}

pub fn handle_interaction<T>(req: &InteractionRequest) -> InteractionResponse
where
    T: InteractionHandler,
{
    match req.r#type {
        InteractionType::Ping => InteractionResponse {
            r#type: InteractionCallbackType::Pong,
            data: None,
        },

        InteractionType::ApplicationCommand => match T::handle_application_command(req.into())
        {
            Response::Message(m) => m.into(),
            Response::Modal(m) => m.into(),
        },

        InteractionType::MessageComponent => match T::handle_message_component(req.into()) {
            Response::Message(m) => m.into(),
            Response::Modal(m) => m.into(),
        },

        InteractionType::ModalSubmit => match T::handle_modal_submit(req.into()) {
            Response::Message(m) => m.into(),
            Response::Modal(_) => panic!("Modal cannot result in another modal!"),
        },
    }
}
