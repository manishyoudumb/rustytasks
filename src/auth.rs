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

            
        }
    }
    

}
