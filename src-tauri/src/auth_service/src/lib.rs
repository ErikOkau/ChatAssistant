pub use auth_utils::login;
pub use auth_utils::models::Credentials;
pub use crate::db::get_user;
pub use db::connect_to_db;

pub use db::Status;

pub mod db;
pub mod auth_utils;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_db() {
        login(creds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticate() {
        let creds = Credentials {
            username: String::from("testuser"),
            password: String::from("password"),
        };
        authenticate(creds);
    }
}