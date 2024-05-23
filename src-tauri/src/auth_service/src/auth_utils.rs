pub mod models;
pub use models::Credentials;
pub use crate::db::get_user;

pub fn login(_creds: Credentials) {
    get_user();
    
}

pub fn logout() {
    println!("User logged out")
}