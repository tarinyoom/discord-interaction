use super::discord_types::*;

/* Request Types */

pub struct ApplicationCommand {
    pub command_name: String,
    pub user_id: String,
}

pub struct MessageComponent {
    pub id: String,
}

pub struct ModalSubmit {
    pub id: String,
}

/* Response Types */

pub struct MessageResponse {
    pub text: String,
    pub buttons: Vec<Button>,
    pub ephemeral: bool,
    pub edit: bool,
}

pub struct Button {
    pub id: String,
    pub text: String,
}

/* Convenience helpers */

impl MessageResponse {
    pub fn new() -> Self {
        MessageResponse {
            text: "".to_string(),
            buttons: Vec::new(),
            ephemeral: false,
            edit: false,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn button(mut self, text: &str, id: &str) -> Self {
        self.buttons.push(Button {
            text: text.to_string(),
            id: id.to_string(),
        });
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.ephemeral = true;
        self
    }

    pub fn edit(mut self) -> Self {
        self.edit = true;
        self
    }
}

/* Conversions to and from Discord types */

impl From<&Request> for ApplicationCommand {
    fn from(req: &Request) -> Self {
        ApplicationCommand {
            command_name: req.data.as_ref().unwrap().name.as_ref().unwrap().clone(),
            user_id: req.member.as_ref().unwrap().user.id.clone(),
        }
    }
}

impl From<&Request> for MessageComponent {
    fn from(req: &Request) -> Self {
        MessageComponent {
            id: req
                .data
                .as_ref()
                .unwrap()
                .custom_id
                .as_ref()
                .unwrap()
                .clone(),
        }
    }
}

impl From<&Request> for ModalSubmit {
    fn from(req: &Request) -> Self {
        ModalSubmit { id: "".to_string() }
    }
}

impl Into<Response> for MessageResponse {
    fn into(self) -> Response {
        let rows = self.buttons
            .chunks(5)
            .map(|chunk| Component {
                r#type: ComponentType::ActionRow,
                label: None,
                style: None,
                custom_id: None,
                value: None,
                components: Some(chunk
                                 .iter()
                                 .map(|b| b.into())
                                 .collect())
            })
            .collect();

        Response {
            r#type: if self.edit {
                InteractionCallbackType::UpdateMessage
            } else {
                InteractionCallbackType::ChannelMessageWithSource
            },

            data: Some(InteractionCallbackData {
                content: Some(self.text),
                components: Some(rows),
                flags: Some(if self.ephemeral { 64 } else { 0 }),
                custom_id: None,
                title: None,
            }),
        }
    }
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
