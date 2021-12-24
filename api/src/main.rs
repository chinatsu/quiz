use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tide_websockets::WebSocket;
use tide::security::{CorsMiddleware, Origin};
use http_types::headers::HeaderValue;


mod api;
mod db;
mod ws;

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    let state = State { pool };

    let cors = CorsMiddleware::new()
    .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    .allow_origin(Origin::from("*"))
    .allow_credentials(false);

    let mut app = tide::with_state(state);
    app.with(cors);
    app.at("/create/quiz").post(api::create_quiz);
    app.at("/create/quiz/:q/question")
        .post(api::create_question);
    app.at("/quiz/:q").get(WebSocket::new(ws::get_quiz));
    app.at("/session/:s/:n")
        .get(WebSocket::new(ws::play_session));
    app.at("/session/new/:q").get(api::new_session);
    app.listen("0.0.0.0:3001").await?;
    Ok(())
}
