use crate::{Request, Response};
use crate::interaction_types::InteractionType;

pub struct InteractionHandler {}

impl InteractionHandler {
    pub fn handle_interaction(&self, req: &Request) -> Response {
        match &req.r#type {
            InteractionType::Ping => Response::pong(),
            _ => Response::message().content("hello world").into()
        }
    }
}
