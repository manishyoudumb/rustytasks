use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::error::Error;

