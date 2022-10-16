mod repo;
mod service;
mod handler;

use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let connection_string = String::from("postgres://localhost/auth");
    // let repo_service = repo::new(connection_string, 5).await?;

    let service = Arc::new(service::new().await?);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(root))
        .route("/users/create", post({
            let user_service = Arc::clone(&service);
            move |body| handler::create_user(body, Arc::clone(&user_service))
        }));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}



async fn app() -> Result<(), sqlx::Error> {
    let connection_string = String::from("postgres://localhost/auth");
    let repo_service = repo::new(connection_string, 5).await?;

    let res = match repo_service.fetch_users().await {
        Ok(users) => {
            users.into_iter().map(|user| {
                format!("ID: {:?}, Username: {:?}, Password: {:?}", user.id, user.username, user.password)
            }).collect()
        }
        Err(y) => vec!(format!("Something went wrong: {}", y))
    };
    println!("RESULT: {:#?}", res);

    let user = repo_service.fetch_user("iamadarshk1".to_owned()).await?;
    println!("USER: {:#?}", user);

    // let _res = repo_service.add_user("iamadarshk".to_owned(), "some_random_password".to_owned()).await?;
    Ok(())}
