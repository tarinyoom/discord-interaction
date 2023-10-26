use super::discord_types::*;
use std::collections::HashMap;

/// An application command is an interaction type consisting of a text command
/// entered into chat, prefixed by a `/` symbol.
pub struct ApplicationCommand {
    /// The name of the command, without the prefix `/`.
    pub command_name: String,

    /// The user id of the user invoking the command.
    pub user_id: String,
}

/// A message component is an interaction type resulting from a button press.
pub struct MessageComponent {
    /// The ID of the component.
    pub id: String,
}

/// A modal submit is an interaction type resulting in a user submitting a
/// pop-up modal form.
pub struct ModalSubmit {
    pub id: String,
    pub values: HashMap<String, String>,
}

/// A response can take one of two forms, either a modal or a message.
pub enum Response {
    Message(Message),
    Modal(Modal),
}

/// A message response will result in a message being posted in the Discord
/// chat. This message will come from directly from the bot.
pub struct Message {
    /// The text content of the message.
    pub text: String,

    /// Buttons attached to the message.
    pub buttons: Vec<Button>,

    /// If this is true, then the message will be only visible to the user who
    /// triggered it.
    pub ephemeral: bool,

    /// If this is true, then the message will edit the original Discord
    /// message that this interaction spawned off of.
    pub edit: bool,
}

/// A button component, which the user can interact with. If a user clicks such
/// a button, it will spawn a message component interaction.
pub struct Button {
    /// The ID of the button, to be passed with any message component
    /// interaction it triggers.
    pub id: String,

    /// The text displayed on the button.
    pub text: String,
}

/// A modal component, which allows the user to input text information. When
/// the user submits the modal, it will spawn a modal submit interaction.
pub struct Modal {
    /// The ID of the modal, to be passed with the modal submit interaction it
    /// triggers.
    pub id: String,

    /// The title text displayed on the modal.
    pub title: String,

    /// A list of text fields included in the modal.
    pub fields: Vec<TextField>,
}

/// A text field included on the modal.
pub struct TextField {
    /// The ID of the text field, to be passed as a key along with any modal
    /// submit interaction.
    pub id: String,

    /// The label text displayed next to the text field.
    pub label: String,
}

/// Convenience methods for building messages.
impl Message {
    /// Creates a new message, defaulting to non-ephemeral, and non-editing.
    pub fn new() -> Self {
        Message {
            text: "".to_string(),
            buttons: Vec::new(),
            ephemeral: false,
            edit: false,
        }
    }

    /// Sets the `text` field on the message.
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Adds a button to the message.
    pub fn button(mut self, id: &str, text: &str) -> Self {
        self.buttons.push(Button {
            id: id.to_string(),
            text: text.to_string(),
        });
        self
    }

    /// Sets the message to be ephemeral.
    pub fn ephemeral(mut self) -> Self {
        self.ephemeral = true;
        self
    }

    /// Sets the message to edit the discord message that spawned it.
    pub fn edit(mut self) -> Self {
        self.edit = true;
        self
    }
}

/// Convenience methods for building modals.
impl Modal {
    /// Creates a new modal.
    pub fn new() -> Self {
        Modal {
            id: "unknown modal".to_string(),
            title: "".to_string(),
            fields: Vec::new(),
        }
    }

    /// Sets the `id` of the modal.
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    /// Sets the `title` of the modal.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Adds a field to the modal.
    pub fn field(mut self, id: &str, label: &str) -> Self {
        self.fields.push(TextField {
            id: id.to_string(),
            label: label.to_string(),
        });
        self
    }
}

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
