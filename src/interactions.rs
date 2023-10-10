/*!
 * Discord interaction request and response types. These are serializable data structures that
 * match the JSON structure established by the Discord API.
 */

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections;

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionRequest {
    pub r#type: InteractionType,
    pub data: Option<InteractionData>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
}

impl InteractionRequest {
    pub fn ping() -> Self {
        InteractionRequest {
            r#type: InteractionType::Ping,
            data: None,
            member: None,
            message: None,
        }
    }

    pub fn get_user(&self) -> String {
        match &self.member {
            Some(m) => m.user.id.clone(),
            None => "Unknown user".to_string(),
        }
    }

    pub fn message_content(&self) -> String {
        match &self.message {
            Some(m) => m.content.clone(),

            None => "".to_string(),
        }
    }

    pub fn command_name(&self) -> Option<String> {
        match &self.data {
            Some(data) => match &data {
                InteractionData::Command(app_data) => Some(app_data.name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn custom_id(&self) -> Option<String> {
        match &self.data {
            Some(data) => match &data {
                InteractionData::Command(_) => None,
                InteractionData::Message(msg_data) => Some(msg_data.custom_id.clone()),
                InteractionData::Modal(modal_data) => Some(modal_data.custom_id.clone()),
            },
            None => None,
        }
    }

    pub fn modal_submit_values(&self) -> collections::HashMap<String, String> {
        match &self.data {
            Some(data) => match &data {
                InteractionData::Modal(modal_data) => modal_data.values(),
                _ => collections::HashMap::new(),
            },
            _ => collections::HashMap::new(),
        }
    }

    pub fn member(mut self, member: GuildMember) -> Self {
        self.member = Some(member);
        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.message = Some(message);
        self
    }

    pub fn application_command(name: &str) -> ApplicationCommandData {
        ApplicationCommandData::new(name)
    }

    pub fn message_component(custom_id: &str, component_type: u8) -> MessageComponentData {
        MessageComponentData::new(custom_id, component_type)
    }
}

impl From<ApplicationCommandData> for InteractionRequest {
    fn from(data: ApplicationCommandData) -> Self {
        InteractionRequest {
            r#type: InteractionType::ApplicationCommand,
            data: Some(InteractionData::Command(data)),
            member: None,
            message: None,
        }
    }
}

impl From<MessageComponentData> for InteractionRequest {
    fn from(data: MessageComponentData) -> Self {
        InteractionRequest {
            r#type: InteractionType::MessageComponent,
            data: Some(InteractionData::Message(data)),
            member: None,
            message: None,
        }
    }
}

impl From<ModalSubmitData> for InteractionRequest {
    fn from(data: ModalSubmitData) -> Self {
        InteractionRequest {
            r#type: InteractionType::ModalSubmit,
            data: Some(InteractionData::Modal(data)),
            member: None,
            message: None,
        }
    }
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ModalSubmit = 5,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum InteractionData {
    Command(ApplicationCommandData),
    Message(MessageComponentData),
    Modal(ModalSubmitData),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ApplicationCommandData {
    name: String,
}

impl ApplicationCommandData {
    pub fn new(name: &str) -> ApplicationCommandData {
        ApplicationCommandData {
            name: name.to_string(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct MessageComponentData {
    custom_id: String,
    component_type: u8,
}

impl MessageComponentData {
    pub fn new(custom_id: &str, component_type: u8) -> Self {
        MessageComponentData {
            custom_id: custom_id.to_string(),
            component_type: component_type,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ModalSubmitData {
    custom_id: String,
    components: Vec<ActionRow>,
}

impl ModalSubmitData {
    pub fn values(&self) -> collections::HashMap<String, String> {
        self.components
            .iter()
            .map(|row| row.component_value())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect()
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GuildMember {
    user: User,
}

impl GuildMember {
    pub fn new(user: &str) -> Self {
        GuildMember {
            user: User {
                id: user.to_string(),
            },
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Message {
    pub content: String,
    pub interaction: Option<MessageInteraction>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct MessageInteraction {
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionResponse {
    r#type: InteractionCallbackType,
    data: InteractionCallbackData,
}

impl InteractionResponse {
    pub fn pong() -> Self {
        let data = MessageCallbackData {
            content: "".to_string(),
            flags: None,
            components: Vec::new(),
        };

        InteractionResponse {
            r#type: InteractionCallbackType::Pong,
            data: InteractionCallbackData::Message(data),
        }
    }

    pub fn message() -> MessageCallbackData {
        MessageCallbackData {
            content: "".to_string(),
            flags: Some(MessageFlags::Ephemeral),
            components: Vec::new(),
        }
    }

    pub fn modal() -> ModalCallbackData {
        ModalCallbackData {
            custom_id: "".to_string(),
            title: "".to_string(),
            components: Vec::new(),
        }
    }

    pub fn edit(mut self) -> Self {
        self.r#type = InteractionCallbackType::UpdateMessage;
        self
    }

    pub fn message_content(&self) -> Option<String> {
        match &self.data {
            InteractionCallbackData::Message(m) => Some(m.content.clone()),
            _ => None,
        }
    }

    pub fn message_components(&self) -> Vec<Component> {
        match &self.data {
            InteractionCallbackData::Message(m) => {
                if m.components.len() != 1 {
                    panic!();
                } else {
                    m.components[0].components.clone()
                }
            }
            _ => vec![],
        }
    }
}

impl From<ModalCallbackData> for InteractionResponse {
    fn from(data: ModalCallbackData) -> InteractionResponse {
        InteractionResponse {
            r#type: InteractionCallbackType::Modal,
            data: InteractionCallbackData::Modal(data),
        }
    }
}

impl From<MessageCallbackData> for InteractionResponse {
    fn from(data: MessageCallbackData) -> InteractionResponse {
        InteractionResponse {
            r#type: InteractionCallbackType::ChannelMessageWithSource,
            data: InteractionCallbackData::Message(data),
        }
    }
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    UpdateMessage = 7,
    Modal = 9,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(MessageCallbackData),
    Modal(ModalCallbackData),
}

#[derive(Serialize, PartialEq, Debug)]
pub struct MessageCallbackData {
    content: String,
    flags: Option<MessageFlags>,
    components: Vec<ActionRow>,
}

impl MessageCallbackData {
    pub fn content(mut self, msg: &str) -> Self {
        self.content = msg.to_string();
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = vec![ActionRow::new().components(components)];
        self
    }

    pub fn shout(mut self) -> Self {
        self.flags = None;
        self
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct ModalCallbackData {
    custom_id: String,
    title: String,
    components: Vec<ActionRow>,
}

impl ModalCallbackData {
    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components
            .iter()
            .map(|c| ActionRow::new().components(vec![c.clone()]))
            .collect();
        self
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ActionRow {
    r#type: ComponentType,
    components: Vec<Component>,
}

impl ActionRow {
    pub fn new() -> Self {
        ActionRow {
            r#type: ComponentType::ActionRow,
            components: Vec::new(),
        }
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn component_value(&self) -> Option<(String, String)> {
        match &self.components[0] {
            Component::Text(text) => text.value(),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Component {
    Button(Button),
    Text(TextInput),
}

impl Component {
    pub fn button() -> Button {
        Button {
            r#type: ComponentType::Button,
            label: None,
            style: ButtonStyle::Primary,
            custom_id: "unlabeled button".to_string(),
        }
    }

    pub fn text_input() -> TextInput {
        TextInput::new()
    }

    pub fn value(&self) -> Option<(String, String)> {
        match self {
            Component::Button(_) => None,
            Component::Text(text) => text.value(),
        }
    }
}

impl From<Button> for Component {
    fn from(button: Button) -> Component {
        Component::Button(button)
    }
}

impl From<TextInput> for Component {
    fn from(text: TextInput) -> Component {
        Component::Text(text)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Button {
    r#type: ComponentType,
    label: Option<String>,
    style: ButtonStyle,
    custom_id: String,
}

impl Button {
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct TextInput {
    r#type: ComponentType,
    label: Option<String>,
    style: Option<TextInputStyle>,
    custom_id: String,
    value: Option<String>,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            r#type: ComponentType::TextInput,
            label: None,
            style: Some(TextInputStyle::Short),
            custom_id: "unlabeled text input".to_string(),
            value: None,
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }

    pub fn value(&self) -> Option<(String, String)> {
        let s = self.custom_id.clone();
        let v = self.value.as_ref()?.clone();
        Some((s, v))
    }
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum TextInputStyle {
    Short = 1,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ComponentType {
    ActionRow = 1,
    Button = 2,
    TextInput = 4,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ButtonStyle {
    Primary = 1,
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
enum MessageFlags {
    Ephemeral = 64,
}
