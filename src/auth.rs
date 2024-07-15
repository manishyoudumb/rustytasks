use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::error::Error;

pub async fn login() -> Result<(), Box<dyn Error>> {
    let client = create_oauth_client()?;

    let (auth_url, _) = client 
        .authorize_url(oauth2::CsrffToken::new_random)
       .add_scope(Scope::new("https://www.googleapis.com/auth/tasks".to_string()))
        .url();

    println!("Please Open this URL in your browser to login:\n\n{}\n\n",auth_url);
    

    

}
