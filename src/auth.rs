use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;
use crate::error::{TodoError, TodoResult};
use tokio::fs;

pub async fn login() -> TodoResult<()> {
    let client = BasicClient::new(
        ClientId::new(std::env::var("GOOGLE_CLIENT_ID").map_err(|_| TodoError::ConfigError("GOOGLE_CLIENT_ID not set".to_string()))?),
        Some(ClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET").map_err(|_| TodoError::ConfigError("GOOGLE_CLIENT_SECRET not set".to_string()))?)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).map_err(|e| TodoError::AuthError(e.to_string()))?,
        Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).map_err(|e| TodoError::AuthError(e.to_string()))?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).map_err(|e| TodoError::AuthError(e.to_string()))?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/tasks".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open this URL in your browser:\n{}\n", auth_url);

    let (code, state) = get_authorization_code()?;

    if state.secret() != csrf_token.secret() {
        return Err(TodoError::AuthError("CSRF token mismatch".to_string()));
    }

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| TodoError::AuthError(e.to_string()))?;

    fs::write("token.json", serde_json::to_string(&token)?).await?;

    println!("Successfully logged in and saved token.");
    Ok(())
}

fn get_authorization_code() -> TodoResult<(AuthorizationCode, CsrfToken)> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            reader.read_line(&mut request_line)?;

            let redirect_url = request_line.split_whitespace().nth(1).ok_or_else(|| TodoError::AuthError("Invalid redirect URL".to_string()))?;
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).map_err(|e| TodoError::AuthError(e.to_string()))?;

            let code_pair = url
                .query_pairs()
                .find(|pair| pair.0 == "code")
                .ok_or_else(|| TodoError::AuthError("No code in the response".to_string()))?;

            let state_pair = url
                .query_pairs()
                .find(|pair| pair.0 == "state")
                .ok_or_else(|| TodoError::AuthError("No state in the response".to_string()))?;

            let message = "You can now close this window.";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;

            return Ok((AuthorizationCode::new(code_pair.1.into_owned()), CsrfToken::new(state_pair.1.into_owned())));
        }
    }
    Err(TodoError::AuthError("Failed to get authorization code".to_string()))
}

pub async fn logout() -> TodoResult<()> {
    fs::remove_file("token.json").await?;
    println!("Logged out successfully");
    Ok(())
}