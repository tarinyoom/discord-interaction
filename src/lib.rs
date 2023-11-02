//! Discord interactions with AWS Lambda!
//!
//! A light-weight, easy-to-use library for building Discord bots under the Discord interaction modal. Integrates with AWS Lambda.

//! Much of a Discord bot's behavior can be described using an request/response model, which a slash command, button press, or other user interaction is sent to a backend, and the backend returns a single response. This request/response is called an *interaction* by Discord, which is specified in detail in the Discord developer docs. This library wraps these request and response types, handling authentication, exposing application-friendly types, and integrating these types with AWS Lambda.

mod auth;
mod discord_types;
mod handler;
mod user_types;

pub use auth::run;
pub use handler::InteractionHandler;
pub use user_types::*;
