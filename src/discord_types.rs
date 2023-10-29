/*!
 * Discord interaction request and response types. These are serializable data structures that
 * match the JSON structure established by the Discord API.
 */

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionRequest {
    pub r#type: InteractionType,
    pub data: Option<InteractionData>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
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
pub struct InteractionData {
    pub name: Option<String>,
    pub custom_id: Option<String>,
    pub components: Option<Vec<Component>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GuildMember {
    pub user: User,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Message {
    pub content: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionResponse {
    pub r#type: InteractionCallbackType,
    pub data: Option<InteractionCallbackData>,
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
pub struct InteractionCallbackData {
    pub content: Option<String>,
    pub flags: Option<u8>,
    pub components: Option<Vec<Component>>,
    pub custom_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Component {
    pub r#type: ComponentType,
    pub label: Option<String>,
    pub style: Option<u8>,
    pub custom_id: Option<String>,
    pub value: Option<String>,
    pub components: Option<Vec<Component>>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TextInputStyle {
    Short = 1,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    TextInput = 4,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
}
