extern crate google_oauth;
use google_oauth::AsyncClient;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let client_id = env::var("GoogleAuth_ClientID").expect("Cannot find GoogleAuth_ClientID in .env file");
    let id_token = env::var("GoogleAuth_ClientSecret").expect("Cannot find GoogleAuth_ClientSecret in .env file");

    let client = AsyncClient::new(client_id);
    /// or, if you want to set the default timeout for fetching certificates from Google, e.g, 30 seconds, you can:
    /// ```rust
    /// let client = AsyncClient::new(client_id).timeout(time::Duration::from_sec(30));
    /// ```

    let payload = client.validate_id_token(id_token).await.unwrap(); // In production, remember to handle this error.

    // When we get the payload, that mean the id_token is valid.
    // Usually we use `sub` as the identifier for our user...
    println!("Hello, I am {}", &payload.sub);
}