use crate::{api, db, State};
use async_std::prelude::*;
use std::{thread, time};
use sqlx::postgres::PgPool;
use tide::prelude::*;
use tide::Request;
use tide_websockets::{Message, WebSocketConnection};

#[derive(Debug, Deserialize, Serialize)]
enum WebsocketMessage {
    Quiz,
    Question,
    Result,
    End,
    PlayerResults,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketQuiz {
    message_type: WebsocketMessage,
    name: String,
    description: String,
    num_questions: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketQuestion {
    message_type: WebsocketMessage,
    index: usize,
    text: String,
    image_url: Option<String>,
    alternatives: Vec<WebsocketAnswer>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketResult {
    message_type: WebsocketMessage,
    index: usize,
    correct: bool,
    score: i32,
    correct_answers: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketPlayerResult {
    message_type: WebsocketMessage,
    players: Vec<db::Player>,
    game_ended: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct IncomingWebsocketAnswer {
    index: usize,
    answer: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketEndResult {
    message_type: WebsocketMessage,
    score: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketAnswer {
    index: usize,
    text: String,
}

fn flat_to_nested_quiz(flat: Vec<db::FlatQuiz>) -> api::OutgoingQuiz {
    let mut quiz = api::OutgoingQuiz {
        name: flat[0].name.clone(),
        description: flat[0].description.clone(),
        quiz_id: flat[0].quiz_id,
        questions: Vec::new(),
    };

    let mut prev_question = &db::FlatQuiz {
        quiz_id: 0,
        name: "".into(),
        description: "".into(),
        question_id: -1,
        question_text: "".into(),
        image_url: None,
        answer_id: 0,
        answer_text: "".into(),
        correct: None,
    };
    let mut ptr = 0usize;
    for line in flat.iter() {
        if prev_question.question_id != line.question_id {
            quiz.questions.push(api::NewQuestion {
                question_text: line.question_text.clone(),
                image_url: line.image_url.clone(),
                answers: Vec::new(),
            });
            ptr += 1;
        }
        quiz.questions[ptr - 1].answers.push(api::NewAnswer {
            answer_text: line.answer_text.clone(),
            correct: line.correct,
        });
        prev_question = line;
    }
    quiz
}

pub async fn play_session(
    req: Request<State>,
    mut stream: WebSocketConnection,
) -> tide::Result<()> {
    let session_id: i32 = req.param("s")?.parse()?;
    let name: String = req.param("n")?.to_string();

    let flat_quiz = sqlx::query_as!(
        db::FlatQuiz,
        r#"
        SELECT 
            a.quiz_id,     a.name,          a.description, 
            b.question_id, b.question_text, b.image_url, 
            c.answer_id,   c.answer_text,   c.correct 
        FROM sessions d
        INNER JOIN quizes a ON (a.quiz_id = d.quiz_id)
        INNER JOIN questions b ON (b.quiz_id = a.quiz_id)
        INNER JOIN answers c ON (c.question_id = b.question_id)
        WHERE d.session_id = $1
        "#,
        session_id
    )
    .fetch_all(&req.state().pool)
    .await?;

    let quiz = flat_to_nested_quiz(flat_quiz);
    
    stream
        .send_json(&WebsocketQuiz {
            message_type: WebsocketMessage::Quiz,
            name: quiz.name.clone(),
            description: quiz.description.clone(),
            num_questions: quiz.questions.len(),
        })
        .await?;
    let player = sqlx::query_as!(
        db::Player,
        r#"
        SELECT * FROM players WHERE session_id = $1 AND name = $2
        "#,
        session_id,
        name
    )
    .fetch_optional(&req.state().pool)
    .await?;

    
    if player.is_none() {
        let new_player = sqlx::query_as!(
            db::Player,
            r#"
        INSERT INTO players (session_id, score, name)
        VALUES ($1, 0, $2)
        RETURNING player_id, session_id, score, finished, name
        "#,
            session_id,
            name
        )
        .fetch_one(&req.state().pool)
        .await?;

        play_quiz(quiz, &mut stream, Some(&new_player), Some(&req.state().pool)).await?;

        db::set_finished(new_player.player_id, &req.state().pool).await?;
    }
    loop {
        let players = sqlx::query_as!(
            db::Player,
            r#"
            SELECT * FROM players WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_all(&req.state().pool)
        .await?;

        let completed = players.iter().all(|p| p.finished);
        stream
            .send_json(&WebsocketPlayerResult {
                message_type: WebsocketMessage::PlayerResults,
                players: players,
                game_ended: completed,
            })
            .await?;
        if completed {
            break;
        }

        thread::sleep(time::Duration::new(1, 0));
    }

    stream.send(Message::Close(None)).await?;

    Ok(())
}

pub async fn get_quiz(req: Request<State>, mut stream: WebSocketConnection) -> tide::Result<()> {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let flat_quiz = sqlx::query_as!(
        db::FlatQuiz,
        r#"
        SELECT 
            a.quiz_id,     a.name,          a.description, 
            b.question_id, b.question_text, b.image_url, 
            c.answer_id,   c.answer_text,   c.correct 
        FROM quizes a
        INNER JOIN questions b ON (b.quiz_id = a.quiz_id)
        INNER JOIN answers c ON (c.question_id = b.question_id)
        WHERE a.quiz_id = $1
        "#,
        quiz_id
    )
    .fetch_all(&req.state().pool)
    .await?;
    let quiz = flat_to_nested_quiz(flat_quiz);

    stream
        .send_json(&WebsocketQuiz {
            message_type: WebsocketMessage::Quiz,
            name: quiz.name.clone(),
            description: quiz.description.clone(),
            num_questions: quiz.questions.len(),
        })
        .await?;

    play_quiz(quiz, &mut stream, None, None).await?;
    stream.send(Message::Close(None)).await?;

    Ok(())
}

async fn play_quiz(quiz: api::OutgoingQuiz, stream: &mut WebSocketConnection, player: Option<&db::Player>, pool: Option<&PgPool>) -> tide::Result<()> {
    let mut score = 0i32;

    for (idx, question) in quiz.questions.iter().enumerate() {
        stream
            .send_json(&WebsocketQuestion {
                message_type: WebsocketMessage::Question,
                index: idx + 1,
                text: question.question_text.clone(),
                image_url: question.image_url.clone(),
                alternatives: question
                    .answers
                    .iter()
                    .enumerate()
                    .map(|(i, a)| WebsocketAnswer {
                        index: i + 1,
                        text: a.answer_text.clone(),
                    })
                    .collect(),
            })
            .await?;

        let correct_answers: Vec<usize> = question
            .answers
            .iter()
            .enumerate()
            .filter(|(_, ans)| match ans.correct {
                Some(correct) => correct,
                None => false,
            })
            .map(|(i, _)| i + 1)
            .collect();

        let submitted_answer: IncomingWebsocketAnswer;

        loop {
            let ans: Option<IncomingWebsocketAnswer> = match stream.next().await {
                Some(Ok(Message::Text(input))) => Some(serde_json::from_str(&input)?),
                _ => None,
            };
            if let Some(answer) = ans {
                submitted_answer = answer;
                break;
            }
        }

        if submitted_answer.answer == 0
            || submitted_answer.answer > question.answers.len()
            || submitted_answer.index != idx + 1
        {
            stream
                .send_string("This is supposed to never happen".into())
                .await?;
            continue;
        }

        let correct_answer = correct_answers.contains(&submitted_answer.answer);
        if correct_answer {
            score += 1;
            if player.is_some() && pool.is_some() {
                db::update_score(player.unwrap().player_id, score, pool.unwrap()).await?;
            }
        }

        stream
            .send_json(&WebsocketResult {
                message_type: WebsocketMessage::Result,
                index: idx + 1,
                correct: correct_answer,
                score: score,
                correct_answers: correct_answers,
            })
            .await?;

        thread::sleep(time::Duration::new(3, 0));
    }
    stream
        .send_json(&WebsocketEndResult {
            message_type: WebsocketMessage::End,
            score: score,
        })
        .await?;
    Ok(())
}
