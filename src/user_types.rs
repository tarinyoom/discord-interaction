use super::discord_types;
use std::collections::HashMap;

/// An top level interaction initiated by the user. Application commands do not require any existing conversation with the bot to be occurring. Currently, only chat application commands (slash commands) are fully supported.
pub struct ApplicationCommand {
    pub command_name: String,
    pub user_id: String,
}

/// An interaction caused by the user's interaction with a message component embedded in a chat message. Currently, only button presses are supported.
pub struct MessageComponent {
    pub id: String,

    /// The message that this component was originally attached to.
    pub source: SourceMessage,
}

/// An interaction type caused by the user submitting a completed modal form. Modals are the
/// primary way of retrieving text input from the user.
pub struct ModalSubmit {
    pub id: String,
    pub values: HashMap<String, String>,
    /// The message that this modal was originally attached to.
    pub source: SourceMessage,
}

/// A message that a message component or modal was originally attached to. This allows the
/// application to maintain some notion of "state", by reasoning based on the source message's
/// text.
pub struct SourceMessage {
    pub text: String,
}

/// A response to an interaction. This response can either be a message in chat, or a modal, which
/// will pop up over the user's screen.
pub enum Response {
    Message(Message),
    Modal(Modal),
}

/// A message response, resulting in a message in chat.
pub struct Message {
    pub text: String,
    pub buttons: Vec<Button>,
    /// If true, the message will be visible to only the recipient.
    pub ephemeral: bool,
    /// If true, the message will replace the original message.
    pub edit: bool,
}

/// A button component, which the user can interact with. If a user clicks such
/// a button, it will spawn a message component interaction.
pub struct Button {
    pub id: String,
    pub text: String,
}

/// A modal response, which allows the user to input text information. A modal cannot be a response
/// to a modal submit interaction.
pub struct Modal {
    pub id: String,
    pub title: String,
    pub fields: Vec<TextField>,
}

/// A text field included in a modal.
pub struct TextField {
    pub id: String,
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

impl From<&discord_types::InteractionRequest> for ApplicationCommand {
    fn from(req: &discord_types::InteractionRequest) -> Self {
        ApplicationCommand {
            command_name: req.data.as_ref().unwrap().name.as_ref().unwrap().clone(),
            user_id: req.member.as_ref().unwrap().user.id.clone(),
        }
    }
}

impl From<&discord_types::InteractionRequest> for MessageComponent {
    fn from(req: &discord_types::InteractionRequest) -> Self {
        MessageComponent {
            id: req
                .data
                .as_ref()
                .unwrap()
                .custom_id
                .as_ref()
                .unwrap()
                .clone(),

            source: req.message.as_ref().unwrap().into(),
        }
    }
}

impl From<&discord_types::InteractionRequest> for ModalSubmit {
    fn from(req: &discord_types::InteractionRequest) -> Self {
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

            source: req.message.as_ref().unwrap().into(),
        }
    }
}

impl From<&discord_types::Message> for SourceMessage {
    fn from(msg: &discord_types::Message) -> Self {
        SourceMessage {
            text: msg.content.clone(),
        }
    }
}

impl Into<discord_types::InteractionResponse> for Message {
    fn into(self) -> discord_types::InteractionResponse {
        let rows = self
            .buttons
            .chunks(5)
            .map(|chunk| discord_types::Component {
                r#type: discord_types::ComponentType::ActionRow,
                label: None,
                style: None,
                custom_id: None,
                value: None,
                components: Some(chunk.iter().map(|b| b.into()).collect()),
            })
            .collect();

        discord_types::InteractionResponse {
            r#type: if self.edit {
                discord_types::InteractionCallbackType::UpdateMessage
            } else {
                discord_types::InteractionCallbackType::ChannelMessageWithSource
            },

            data: Some(discord_types::InteractionCallbackData {
                content: Some(self.text),
                components: Some(rows),
                flags: Some(if self.ephemeral { 64 } else { 0 }),
                custom_id: None,
                title: None,
            }),
        }
    }
}

impl Into<discord_types::InteractionResponse> for Modal {
    fn into(self) -> discord_types::InteractionResponse {
        let fields = self
            .fields
            .iter()
            .map(|field| discord_types::Component {
                r#type: discord_types::ComponentType::ActionRow,
                label: None,
                style: None,
                custom_id: None,
                value: None,
                components: Some(vec![discord_types::Component {
                    r#type: discord_types::ComponentType::TextInput,
                    label: Some(field.label.clone()),
                    style: Some(1),
                    custom_id: Some(field.id.clone()),
                    value: None,
                    components: None,
                }]),
            })
            .collect();

        let data = discord_types::InteractionCallbackData {
            content: None,
            flags: None,
            components: Some(fields),
            custom_id: Some(self.id),
            title: Some(self.title),
        };

        discord_types::InteractionResponse {
            r#type: discord_types::InteractionCallbackType::Modal,
            data: Some(data),
        }
    }
}

impl Into<discord_types::Component> for &Button {
    fn into(self) -> discord_types::Component {
        discord_types::Component {
            r#type: discord_types::ComponentType::Button,
            label: Some(self.text.clone()),
            style: Some(1),
            custom_id: Some(self.id.clone()),
            value: None,
            components: None,
        }
    }
}
