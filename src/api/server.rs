use std::convert::Infallible;
use std::net::SocketAddr;
use axum::{Router, routing::get, handler::Handler};

pub struct Server {
    router: Router,
    addr: SocketAddr,
}

impl Server {
    pub async fn new() -> Result<Server, Infallible> {
        tracing::info!("Creating router...");
        let router = Router::new()
            .fallback(
                self::Server::not_found.into_service()
            )
            .route("/areyouok", get(self::Server::healthcheck));
        
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
        let server = Server{router, addr};

        Ok(server)
    }

    pub async fn run(self) {
        tracing::info!("Listening on {}", &self.addr);

        axum::Server::bind(&self.addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap()
    }
}
