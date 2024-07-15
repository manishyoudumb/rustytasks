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

    let Listener = TcpListener::bind("127.0.0.1:8080")?;
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
    
    ok(())

}

pub async fn logout() -> Result<(), Box<dyn Error>> {
    println!("You've been logged out successfully. \n Have a nice day ahead \n");
    Ok(())
}


