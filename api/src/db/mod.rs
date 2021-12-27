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
    pub started: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub session_id: i32,
    pub player_id: i32,
    pub score: i32,
    pub finished: bool,
    pub name: String,
    pub host: bool,
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

pub async fn set_player_name(player_id: i32, name: String, pool: &PgPool) -> tide::Result<()> {
    sqlx::query!(
        "UPDATE players SET name = $1 WHERE player_id = $2",
        name, player_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn new_player(session_id: i32, pool: &PgPool) -> tide::Result<Player> {
    let new_player = sqlx::query_as!(
        Player,
        r#"
        INSERT INTO players (session_id, score)
        VALUES ($1, 0)
        RETURNING player_id, session_id, score, name, finished, host
        "#,
        session_id
    )
        .fetch_one(pool)
        .await?;

    Ok(new_player)
}

pub async fn get_session(session_id: i32, pool: &PgPool) -> tide::Result<Session> {
    let session = sqlx::query_as!(
        Session,
        r#"
        SELECT * FROM sessions WHERE session_id = $1
        "#,
        session_id
    )
        .fetch_one(pool)
        .await?;

    Ok(session)
}

pub async fn get_session_players(session_id: i32, pool: &PgPool) -> tide::Result<Vec<Player>> {
    let players = sqlx::query_as!(
        Player,
        r#"
        SELECT * FROM players WHERE session_id = $1
        "#,
        session_id
    )
        .fetch_all(pool)
        .await?;

    Ok(players)
}

pub async fn new_session(quiz_id: i32, pool: &PgPool) -> tide::Result<(Session, Player)> {
    let new_sesh = sqlx::query_as!(
        Session,
        r#"
        INSERT INTO sessions (quiz_id)
        VALUES ($1)
        RETURNING session_id, quiz_id, started
        "#,
        quiz_id
    )
        .fetch_one(pool)
        .await?;

    let new_player = sqlx::query_as!(
        Player,
        r#"
        INSERT INTO players (session_id, score, host)
        VALUES ($1, 0, true)
        RETURNING player_id, session_id, score, name, finished, host
        "#,
        new_sesh.session_id
    )
        .fetch_one(pool)
        .await?;

    Ok((new_sesh, new_player))
}