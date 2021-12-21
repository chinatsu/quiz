use async_std::prelude::*;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tide::prelude::*;
use tide::Request;
use tide_websockets::{Message, WebSocket, WebSocketConnection};

#[derive(Debug, Deserialize, Serialize)]
struct InternalQuiz {
    qui_id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct InternalQuestion {
    que_id: String,
    que_text: String,
    image_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct InternalAnswer {
    ans_text: String,
    correct: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Quiz {
    qui_id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Answer {
    ans_id: i32,
    ans_text: String,
    correct: Option<bool>,
    que_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Question {
    que_id: i32,
    que_text: String,
    image_url: Option<String>,
    qui_id: i32,
}

#[derive(Clone)]
struct State {
    pool: PgPool,
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
    //app.at("/questions/").get(get_questions);
    //app.at("/questions/add").post(create_question);
    app.at("/quiz/:q").get(WebSocket::new(get_quiz));
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}

async fn get_quiz(req: Request<State>, mut stream: WebSocketConnection) -> tide::Result<()> {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let quiz = sqlx::query_as!(Quiz, "SELECT * FROM quizes WHERE qui_id = $1", quiz_id)
        .fetch_one(&req.state().pool)
        .await?;
    stream
        .send_string(format!("Welcome to {}!", quiz.name))
        .await?;

    let questions = sqlx::query_as!(
        Question,
        "SELECT * FROM questions WHERE qui_id = $1",
        quiz_id
    )
    .fetch_all(&req.state().pool)
    .await?;

    let mut score = 0u32;

    for (idx, question) in questions.iter().enumerate() {
        stream
            .send_string(format!("Question {}: {}", idx + 1, question.que_text))
            .await?;
        let answers = sqlx::query_as!(
            Answer,
            "SELECT * FROM answers WHERE que_id = $1",
            question.que_id
        )
        .fetch_all(&req.state().pool)
        .await?;

        let correct_answers: Vec<String> = answers
            .iter()
            .filter(|ans| match ans.correct {
                Some(correct) => correct,
                None => false,
            })
            .map(|ans| ans.ans_text.to_ascii_lowercase().to_string())
            .collect();

        for (jdx, ans) in answers.iter().enumerate() {
            stream
                .send_string(format!("\t{}: {}", jdx + 1, ans.ans_text))
                .await?;
        }
        let submitted_answer = match stream.next().await {
            Some(Ok(Message::Text(input))) => input.trim().to_string(),
            Some(_) => "".into(),
            None => "".into(),
        };
        if correct_answers.contains(&submitted_answer.to_ascii_lowercase()) {
            score += 1;
            stream
                .send_string("That's the right answer!".into())
                .await?;
        } else {
            let answer_list = correct_answers.join(" or ");
            stream
                .send_string(format!(
                    "That's is the wrong answer! The right answer was {}",
                    answer_list
                ))
                .await?;
        }
    }
    stream
        .send_string(format!(
            "Thanks for playing! You scored {} out of {}",
            score,
            questions.len()
        ))
        .await?;
    stream.send(Message::Close(None)).await?;
    Ok(())
}

/* async fn create_question(mut req: Request<State>) -> tide::Result {
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
} */
