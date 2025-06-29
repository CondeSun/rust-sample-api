// src/api/mod.rs
use crate::{
    domain::user::{User, UserRepo},
    services::user::UserService,
};
use axum::{Router, routing::get};

pub async fn start_server(
    cfg: crate::config::Config,
    db: impl UserRepo,
) -> Result<(), hyper::Error> {
    let user_service = UserService::new(db);
    let app = Router::new()
        .route("/health", get(health))
        .route("/users/:id", get(user_id));
    axum::Server::bind(&([0, 0, 0, 0], cfg.server.port).into())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn user_id(id: uuid::Uuid, service: UserService<impl UserRepo>) -> User {
    service.get_user(id).await.unwrap()
}

async fn health() -> &'static str {
    "OK"
}
