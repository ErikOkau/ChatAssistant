pub mod models;
use models::Credentials;
use crate::db::get_user;

pub fn login(creds: Credentials) {
    get_user();
    
}

pub(crate) fn logout() {
    
}
