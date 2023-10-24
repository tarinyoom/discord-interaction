use super::discord_types::{Request, Response};

pub struct ApplicationCommand {
    pub name: String,
}

impl From<&Request> for ApplicationCommand {
    fn from(_: &Request) -> Self {
        ApplicationCommand {
            name: "DEBUG_COMMAND_NAME".to_string(),
        }
    }
}

pub struct MessageComponent {
    pub id: String,
}

impl From<&Request> for MessageComponent {
    fn from(req: &Request) -> Self {
        MessageComponent {
            id: req.custom_id().unwrap().clone(),
        }
    }
}

pub struct ModalSubmit {
    pub id: String,
}

impl From<&Request> for ModalSubmit {
    fn from(req: &Request) -> Self {
        ModalSubmit {
            id: req.custom_id().unwrap().clone(),
        }
    }
}

pub struct ApplicationCommandResponse {
    pub text: String,
}

impl Into<Response> for ApplicationCommandResponse {
    fn into(self) -> Response {
        Response::message().content("TEST RESPONSE").into()
    }
}

pub struct MessageComponentResponse {
}

impl Into<Response> for MessageComponentResponse {
    fn into(self) -> Response {
        Response::pong()
    }
}

pub struct ModalSubmitResponse {
}

impl Into<Response> for ModalSubmitResponse {
    fn into(self) -> Response {
        Response::pong()
    }
}
