use super::discord_types::{
    InteractionCallbackType, InteractionRequest, InteractionResponse, InteractionType,
};
use super::user_types::{ApplicationCommand, MessageComponent, ModalSubmit, Response};

/// General interaction handler type, to be implemented by your application. To implement this trait, you must at minimum be able to handle incoming application commands (slash commands). If your application involves buttons or modal inputs, you should implement the corresponding trait functions as well.
pub trait InteractionHandler {
    #[allow(unused)]
    fn handle_application_command(ac: ApplicationCommand) -> Response;

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

        InteractionType::ApplicationCommand => match T::handle_application_command(req.into()) {
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
