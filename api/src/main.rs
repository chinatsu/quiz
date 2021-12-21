use tide::Request;
use async_std::prelude::*;
use tide::prelude::*;
use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;
use std::env;
use tide_websockets::{Message, WebSocket, WebSocketConnection};


#[derive(Debug, Deserialize, Serialize)]
struct Question {
    question: String,
    answer: i32,
}

#[derive(Clone)]
struct State {
    pool: PgPool
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?).await?;
    let state = State { pool };

    let mut app = tide::with_state(state);
    app.at("/questions/").get(get_questions);
    app.at("/questions/add").post(create_question);
    app.at("/quiz/").get(WebSocket::new(get_quiz));
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}

async fn get_quiz(_req: Request<State>, mut stream: WebSocketConnection) -> tide::Result<()> {
    stream.send_string(format!("Welcome to the quiz!")).await?;
    while let Some(Ok(Message::Text(input))) = stream.next().await {
        let output: String = input.chars().rev().collect();
        stream
            .send_string(format!("{} | {}", &input, &output))
            .await?;
    }

    Ok(())
}

async fn create_question(mut req: Request<State>) -> tide::Result {
    let question: Question = req.body_json().await?;

    let new_question = sqlx::query_as!(
        Question,
        r#"
        INSERT INTO question (question, answer)
        VALUES ($1, $2)
        RETURNING question, answer
        "#,
        question.question,
        question.answer,
    )
    .fetch_one(&req.state().pool)
    .await?;

    Ok(json!(new_question).into())
}

async fn get_questions(req: Request<State>) -> tide::Result {
    let questions = sqlx::query_as!(Question, "SELECT * FROM question")
            .fetch_all(&req.state().pool)
        .await?;
    
    Ok(json!(questions).into())
}