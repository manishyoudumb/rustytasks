use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::error::Error;

pub async fn login() -> Result<(), Box<dyn Error>> {
    let client = create_oauth_client()?;

    let (auth_url, _) = client 
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/tasks".to_string()))
        .url();

    println!("Please Open this URL in your browser to login:\n\n{}\n\n",auth_url);

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code; 
            {
                let mut reader = BufReader::new(&stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line)?;

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = url::Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = value.into_owned();
            }

            let message = "You can now close this window and return to the terminal.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;

            let token = client
                .exchange_code(oauth2::AuthorizationCode::new(code))
                .request_async(async_http_client)
                .await?;

            println!("Access token: {}", token.access_token().secret());
            println!("Refresh token: {:?}", token.refresh_token());

            break;
        }
    }
    
    Ok(())

}

pub async fn logout() -> Result<(), Box<dyn Error>> {
    println!("You've been logged out successfully. \n Have a nice day ahead \n");
    Ok(())
}


fn create_oauth_client() -> Result<BasicClient, Box<dyn Error>> {
    let google_client_id = ClientId::new(
        std::env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );

    let google_client_secret = ClientSecret::new(
        std::env::var("GOOGLE_CLIENT_SECRET").expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    Ok(client)
}


//////////////// TEST CASE //////////////// 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn test_create_oauth_client() {
            std::env::set_var("GOOGLE_CLIENT_ID", "test_client_id");
            std::env::set_var("GOOGLE_CLIENT_SECRET", "test_client_secret");

            let client = create_oauth_client().unwrap();

            assert_eq!(client.client_id().as_str(), "test_client_id");
            assert_eq!(client.auth_url().as_str(), "https://accounts.google.com/o/oauth2/v2/auth");
            assert_eq!(client.token_url().unwrap().as_str(), "https://www.googleapis.com/oauth2/v3/token");
            assert_eq!(client.redirect_url().unwrap().as_str(), "http://localhost:8080");
        }


    }