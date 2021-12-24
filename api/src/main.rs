use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tide_websockets::WebSocket;

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

    let mut app = tide::with_state(state);
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
