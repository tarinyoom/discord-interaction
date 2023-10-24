mod auth;
mod discord_types;
mod handler;
mod user_types;

pub use auth::run_handler;
pub use handler::InteractionHandler;
pub use user_types::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
