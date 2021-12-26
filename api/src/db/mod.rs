use tide::prelude::*;
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct Quiz {
    pub quiz_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    pub answer_id: i32,
    pub answer_text: String,
    pub correct: Option<bool>,
    pub question_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub question_id: i32,
    pub question_text: String,
    pub image_url: Option<String>,
    pub quiz_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlatQuiz {
    pub quiz_id: i32,
    pub name: String,
    pub description: String,
    pub question_id: i32,
    pub question_text: String,
    pub image_url: Option<String>,
    pub answer_id: i32,
    pub answer_text: String,
    pub correct: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub session_id: i32,
    pub quiz_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub session_id: i32,
    pub player_id: i32,
    pub score: i32,
    pub finished: bool,
    pub name: String,
}

pub async fn update_score(player_id: i32, score: i32, pool: &PgPool) -> tide::Result<()> {
    sqlx::query!(
        "UPDATE players SET score = $1 WHERE player_id = $2",
        score,
        player_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_finished(player_id: i32, pool: &PgPool) -> tide::Result<()> {
    sqlx::query!(
        "UPDATE players SET finished = true WHERE player_id = $1",
        player_id
    )
    .execute(pool)
    .await?;

    Ok(())
}