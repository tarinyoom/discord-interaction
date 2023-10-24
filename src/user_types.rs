use super::discord_types::*;

pub struct ApplicationCommand {
    pub command: String,
    pub user_id: String,
}

impl From<&Request> for ApplicationCommand {
    fn from(req: &Request) -> Self {
        ApplicationCommand {
            command: req.data.as_ref().unwrap().name.as_ref().unwrap().clone(),
            user_id: req.member.as_ref().unwrap().user.id.clone(),
        }
    }
}

pub struct MessageComponent {
    pub id: String,
}

impl From<&Request> for MessageComponent {
    fn from(req: &Request) -> Self {
        MessageComponent { id: "".to_string() }
    }
}

pub struct ModalSubmit {
    pub id: String,
}

impl From<&Request> for ModalSubmit {
    fn from(req: &Request) -> Self {
        ModalSubmit { id: "".to_string() }
    }
}

pub struct ApplicationCommandResponse {
    pub text: String,
}

impl Into<Response> for ApplicationCommandResponse {
    fn into(self) -> Response {
        Response {
            r#type: InteractionCallbackType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData {
                content: Some(self.text),
                components: Vec::new(),
                flags: Some(MessageFlags::Ephemeral),
                custom_id: None,
                title: None,
            }),
        }
    }
}

pub struct MessageComponentResponse {}

impl Into<Response> for MessageComponentResponse {
    fn into(self) -> Response {
        todo!();
    }
}

pub struct ModalSubmitResponse {}

impl Into<Response> for ModalSubmitResponse {
    fn into(self) -> Response {
        todo!();
    }
}
