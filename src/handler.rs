use crate::{Request, Response};

pub struct InteractionHandler {}

impl InteractionHandler {
    pub fn handle_interaction(&self, _: &Request) -> Response {
        Response::message().content("mega doo doo").into()
    }
}
