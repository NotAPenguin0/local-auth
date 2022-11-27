pub mod port;
pub mod error;

use anyhow;
use hyper;

use std::convert::Infallible;
use std::sync::Arc;
use hyper::{Request, Body, Response};
use futures::channel::mpsc::Sender;
use futures::{Stream, StreamExt};
use crate::error::Error;

/// Local HTTP server that listens for authentication requests on a local port.
pub struct AuthListener {
    address: std::net::SocketAddr
}

impl AuthListener {
    /// Create a new listener, listening to requests on the URL `localhost:{port}`
    pub async fn new(port: port::Port) -> anyhow::Result<Self> {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port.into()));
        Ok(AuthListener {
            address: addr
        })
    }

    /// Listen for messages, and return the first oauth authentication code received.
    pub async fn listen(&mut self) -> anyhow::Result<String> {
        // Sender/Receiver channel for our authentication code.
        let (sender, mut receiver) = futures::channel::mpsc::channel(1);
        let make_service = hyper::service::make_service_fn(move |_conn| {
            let sender = sender.clone();
            let service = hyper::service::service_fn(move |req| {
                AuthListener::request_handler(sender.clone(), req)
            });
            async move { Ok::<_, Infallible>(service) }
        });

        let server = hyper::Server::try_bind(&self.address)?.serve(make_service);
        let mut auth_code: Option<String> = None;
        let server = server.with_graceful_shutdown(async {
            let value = receiver.next().await;
            auth_code = value;
        });

        server.await?;
        auth_code.ok_or(anyhow::Error::new(Error::NoAuthCode))
    }

    async fn request_handler(mut sender: Sender<String>, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let (parts, body) = req.into_parts();
        let uri = parts.uri;
        if let Some(query) = uri.query() {
            // Check if query contains an access code
            if query.contains("code=") {
                // extract key from query
                let key_start = query.find("=").unwrap(); // Cannot fail, we just tested whether it contains 'code='
                let key = &query[key_start+1..];
                // Remove quotes
                let key = key.replace("\"", "");
                // Send it back through our message pipe, this will kill the server
                sender.try_send(key).unwrap();
                return Ok(Response::new("Authentication successful. You can close this browser window.".into()));
            }
        }
        Ok(Response::new("Authentication failed.".into()))
    }
}