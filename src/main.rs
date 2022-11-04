mod api;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting server");
    api::server::Server::new()
        .await?
        .run()
        .await;
    
    Ok(())
}
