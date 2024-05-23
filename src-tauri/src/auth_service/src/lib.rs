
pub use auth_utils::login;
pub use auth_utils::models::Credentials;
pub use crate::db::get_user;
pub use db::connect_to_db;

use db::Status;
use models::Credentials;

mod db;
mod auth_utils;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_db() {
        login(creds);
    }
}
