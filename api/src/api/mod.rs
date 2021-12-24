use crate::{State, db};
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAnswer {
    pub ans_text: String,
    pub correct: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewQuestion {
    pub que_text: String,
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
    pub qui_id: i32,
    pub name: String,
    pub description: String,
    pub questions: Vec<NewQuestion>,
}

pub async fn create_quiz(mut req: Request<State>) -> tide::Result {
    let quiz: NewQuiz = req.body_json().await?;

    let new_quiz = sqlx::query_as!(
        db::Quiz,
        r#"
        INSERT INTO quizes (name, description)
        VALUES ($1, $2)
        RETURNING qui_id, name, description
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
        INSERT INTO questions (que_text, image_url, qui_id)
        VALUES ($1, $2, $3)
        RETURNING que_id, que_text, image_url, qui_id
        "#,
        question.que_text,
        question.image_url,
        quiz_id
    )
    .fetch_one(&req.state().pool)
    .await?;

    for answer in question.answers {
        sqlx::query!(
            r#"
            INSERT INTO answers (ans_text, correct, que_id)
            VALUES ($1, $2, $3)
            "#,
            answer.ans_text,
            answer.correct,
            new_question.que_id
        )
        .execute(&req.state().pool)
        .await?;
    }

    Ok(json!(new_question).into())
}

pub async fn get_answers(req: Request<State>) -> tide::Result {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let quiz = sqlx::query_as!(
        db::Quiz,
        "SELECT * FROM quizes WHERE qui_id = $1",
        quiz_id
    )
    .fetch_one(&req.state().pool)
    .await?;

    let questions = sqlx::query_as!(
        db::Question,
        "SELECT * FROM questions WHERE qui_id = $1",
        quiz_id
    )
    .fetch_all(&req.state().pool)
    .await?;
    let mut internal_questions = Vec::new();
    for q in questions.iter() {
        let answers = sqlx::query_as!(
            db::Answer,
            "SELECT * FROM answers WHERE que_id = $1",
            q.que_id
        )
        .fetch_all(&req.state().pool)
        .await?;

        internal_questions.push(NewQuestion {
            que_text: q.que_text.clone(),
            image_url: q.image_url.clone(),
            answers: answers
            .iter()
            .map(|a| NewAnswer {
                ans_text: a.ans_text.clone(),
                correct: a.correct
            }).collect()
    
        });
    }

    let outgoing_quiz = OutgoingQuiz {
        qui_id: quiz.qui_id,
        name: quiz.name,
        description: quiz.description,
        questions: internal_questions
    };

    Ok(json!(outgoing_quiz).into())
}