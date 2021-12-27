use crate::{db, State};
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAnswer {
    pub answer_text: String,
    pub correct: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewQuestion {
    pub question_text: String,
    pub image_url: Option<String>,
    pub answers: Vec<NewAnswer>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NewQuiz {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingQuiz {
    pub quiz_id: i32,
    pub name: String,
    pub description: String,
    pub questions: Vec<NewQuestion>,
}

pub async fn new_session(req: Request<State>) -> tide::Result {
    let quiz_id: i32 = req.param("q")?.parse()?;

    let new_session = sqlx::query_as!(
        db::Session,
        r#"
        INSERT INTO sessions (quiz_id)
        VALUES ($1)
        RETURNING session_id, quiz_id, started
        "#,
        quiz_id
    )
    .fetch_one(&req.state().pool)
    .await?;

    Ok(json!(new_session).into())
}

pub async fn list_sessions(req: Request<State>) -> tide::Result {
    let sessions = sqlx::query_as!(
        db::Session,
        r#"
        SELECT DISTINCT s.session_id, s.quiz_id, s.started FROM sessions s
        INNER JOIN players p ON p.session_id = s.session_id
        WHERE p.finished = 'false'
        "#
    )   .fetch_all(&req.state().pool)
        .await?;

    Ok(json!(sessions).into())
}

pub async fn create_quiz(mut req: Request<State>) -> tide::Result {
    let quiz: NewQuiz = req.body_json().await?;

    let new_quiz = sqlx::query_as!(
        db::Quiz,
        r#"
        INSERT INTO quizes (name, description)
        VALUES ($1, $2)
        RETURNING quiz_id, name, description
        "#,
        quiz.name,
        quiz.description,
    )
    .fetch_one(&req.state().pool)
    .await?;

    Ok(json!(new_quiz).into())
}

pub async fn create_question(mut req: Request<State>) -> tide::Result {
    let question: NewQuestion = req.body_json().await?;
    let quiz_id: i32 = req.param("q")?.parse()?;

    let new_question = sqlx::query_as!(
        db::Question,
        r#"
        INSERT INTO questions (question_text, image_url, quiz_id)
        VALUES ($1, $2, $3)
        RETURNING question_id, question_text, image_url, quiz_id
        "#,
        question.question_text,
        question.image_url,
        quiz_id
    )
    .fetch_one(&req.state().pool)
    .await?;

    for answer in question.answers {
        sqlx::query!(
            r#"
            INSERT INTO answers (answer_text, correct, question_id)
            VALUES ($1, $2, $3)
            "#,
            answer.answer_text,
            answer.correct,
            new_question.question_id
        )
        .execute(&req.state().pool)
        .await?;
    }

    Ok(json!(new_question).into())
}
