use super::discord_types::*;
use std::collections::HashMap;

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
    pub values: HashMap<String, String>,
}

/* Response Types */

pub enum Response {
    Message(Message),
    Modal(Modal),
}

pub struct Message {
    pub text: String,
    pub buttons: Vec<Button>,
    pub ephemeral: bool,
    pub edit: bool,
}

pub struct Button {
    pub id: String,
    pub text: String,
}

pub struct Modal {
    pub id: String,
    pub title: String,
    pub fields: Vec<TextField>,
}

pub struct TextField {
    pub id: String,
    pub label: String,
}

/* Convenience wrappers */

impl Message {
    pub fn new() -> Self {
        Message {
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

    pub fn button(mut self, id: &str, text: &str) -> Self {
        self.buttons.push(Button {
            id: id.to_string(),
            text: text.to_string(),
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

impl Modal {
    pub fn new() -> Self {
        Modal {
            id: "unknown modal".to_string(),
            title: "".to_string(),
            fields: Vec::new(),
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn field(mut self, id: &str, label: &str) -> Self {
        self.fields.push(TextField {
            id: id.to_string(),
            label: label.to_string(),
        });
        self
    }
}

/* Conversions to and from Discord types */

impl From<&InteractionRequest> for ApplicationCommand {
    fn from(req: &InteractionRequest) -> Self {
        ApplicationCommand {
            command_name: req.data.as_ref().unwrap().name.as_ref().unwrap().clone(),
            user_id: req.member.as_ref().unwrap().user.id.clone(),
        }
    }
}

impl From<&InteractionRequest> for MessageComponent {
    fn from(req: &InteractionRequest) -> Self {
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

impl From<&InteractionRequest> for ModalSubmit {
    fn from(req: &InteractionRequest) -> Self {
        ModalSubmit {
            id: req
                .data
                .as_ref()
                .unwrap()
                .custom_id
                .as_ref()
                .unwrap()
                .clone(),

            values: req
                .data
                .as_ref()
                .unwrap()
                .components
                .as_ref()
                .unwrap()
                .iter()
                .map(|row| {
                    let inner = &row.components.as_ref().unwrap()[0];
                    (
                        inner.custom_id.as_ref().unwrap().clone(),
                        inner.value.as_ref().unwrap().clone(),
                    )
                })
                .collect(),
        }
    }
}

impl Into<InteractionResponse> for Message {
    fn into(self) -> InteractionResponse {
        let rows = self
            .buttons
            .chunks(5)
            .map(|chunk| Component {
                r#type: ComponentType::ActionRow,
                label: None,
                style: None,
                custom_id: None,
                value: None,
                components: Some(chunk.iter().map(|b| b.into()).collect()),
            })
            .collect();

        InteractionResponse {
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

impl Into<InteractionResponse> for Modal {
    fn into(self) -> InteractionResponse {
        let fields = self
            .fields
            .iter()
            .map(|field| Component {
                r#type: ComponentType::ActionRow,
                label: None,
                style: None,
                custom_id: None,
                value: None,
                components: Some(vec![Component {
                    r#type: ComponentType::TextInput,
                    label: Some(field.label.clone()),
                    style: Some(1),
                    custom_id: Some(field.id.clone()),
                    value: None,
                    components: None,
                }]),
            })
            .collect();

        let data = InteractionCallbackData {
            content: None,
            flags: None,
            components: Some(fields),
            custom_id: Some(self.id),
            title: Some(self.title),
        };

        InteractionResponse {
            r#type: InteractionCallbackType::Modal,
            data: Some(data),
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
