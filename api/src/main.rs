use tide::Request;
use tide::prelude::*;
use sqlx::prelude::*;
use sqlx::postgres::Postgres;
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};


#[derive(Debug, Deserialize, Serialize)]
struct Question {
    question: String,
    answer: i32,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.with(SQLxMiddleware::<Postgres>::new("postgres://postgres:hunter2@localhost:2345/quiz").await?);
    app.at("/questions/").get(get_questions);
    app.at("/questions/add").post(create_question);
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}

async fn create_question(mut req: Request<()>) -> tide::Result {
    let question: Question = req.body_json().await?;
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

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
    .fetch_one(pg_conn.acquire().await?)
    .await?;

    Ok(json!(new_question).into())
}

async fn get_questions(req: Request<()>) -> tide::Result {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let questions = sqlx::query_as!(Question, "SELECT * FROM question")
        .fetch_all(pg_conn.acquire().await?)
        .await?;
    
    Ok(json!(questions).into())
}