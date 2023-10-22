use crate::{Request, Response};

pub trait InteractionHandler {
    fn handle_interaction(&self, req: &Request) -> Response;
}
