mod port;

use anyhow;
use hyper;

use std::convert::Infallible;
use hyper::server::conn::AddrIncoming;
use hyper::{Request, Body, Response};

/// Local HTTP server that listens for authentication requests on a local port.
pub struct AuthListener {
    address: std::net::SocketAddr,
}

async fn service_handler_temp(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
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
    pub async fn listen(&self) -> anyhow::Result<()> {
        let make_service = hyper::service::make_service_fn(|_conn| async {
            Ok::<_, Infallible>(hyper::service::service_fn(service_handler_temp))
        });

        let server = hyper::Server::try_bind(&self.address)?.serve(make_service);
        server.await;
        Ok(())
    }
}