use google_oauth::AsyncClient;
use dotenv::dotenv;
use std::env;

fn main() {
    let client_id = env::var("GoogleAuth_ClientID").expect("Cannot find GoogleAuth_ClientID in .env file");
    let id_token = env::var("GoogleAuth_ClientSecret").expect("Cannot find GoogleAuth_ClientSecret in .env file");

    let client = AsyncClient::new(client_id);

    let payload = client.validate_id_token(id_token).await.unwrap(); 

    println!("Hello, I am {}", &payload.sub);
}

//  rustc test_auth.rs