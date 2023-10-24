use super::discord_types::*;

pub struct ApplicationCommand {
    pub command_name: String,
    pub user_id: String,
}

impl From<&Request> for ApplicationCommand {
    fn from(req: &Request) -> Self {
        ApplicationCommand {
            command_name: req.data.as_ref().unwrap().name.as_ref().unwrap().clone(),
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
    pub buttons: Vec<Button>,
}

impl Into<Response> for ApplicationCommandResponse {
    fn into(self) -> Response {
        let row = Component {
            r#type: ComponentType::ActionRow,
            label: None,
            style: None,
            custom_id: None,
            value: None,
            components: Some(self.buttons.iter().map(|b| b.into()).collect()),
        };

        let rows = vec![row];

        Response {
            r#type: InteractionCallbackType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData {
                content: Some(self.text),
                components: rows,
                flags: Some(MessageFlags::Ephemeral),
                custom_id: None,
                title: None,
            }),
        }
    }
}

pub struct Button {
    pub id: String,
    pub text: String,
}

impl Into<Component> for &Button {
    fn into(self) -> Component {
        Component {
            r#type: ComponentType::Button,
            label: Some(self.text.clone()),
            style: Some(1),
            custom_id: Some(self.id.clone()),
            value: None,
            components: None,
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
