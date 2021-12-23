use async_std::prelude::*;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, thread, time};
use tide::prelude::*;
use tide::Request;
use tide_websockets::{Message, WebSocket, WebSocketConnection};


#[derive(Debug, Deserialize, Serialize)]
enum WebsocketMessage {
    Quiz,
    Question,
    Result,
    End,
}

#[derive(Debug, Deserialize, Serialize)]
struct OutgoingQuiz {
    qui_id: i32,
    name: String,
    description: String,
    questions: Vec<IncomingQuestion>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketQuiz {
    message_type: WebsocketMessage,
    name: String,
    description: String,
    num_questions: usize
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketQuestion {
    message_type: WebsocketMessage,
    index: usize,
    text: String,
    image_url: Option<String>,
    alternatives: Vec<WebsocketAnswer>
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketResult {
    message_type: WebsocketMessage,
    index: usize,
    correct: bool,
    score: u32,
    correct_answers: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IncomingWebsocketAnswer {
    index: usize,
    answer: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketEndResult {
    message_type: WebsocketMessage,
    score: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebsocketAnswer {
    index: usize,
    text: String
}

#[derive(Debug, Deserialize, Serialize)]
struct IncomingAnswer {
    ans_text: String,
    correct: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IncomingQuestion {
    que_text: String,
    image_url: Option<String>,
    answers: Vec<IncomingAnswer>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IncomingQuiz {
    name: String,
    description: String,
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
    app.at("/create/quiz").post(create_quiz);
    app.at("/create/quiz/:q/question").post(create_question);
    app.at("/quiz/:q/answers").get(get_answers);
    app.at("/quiz/:q").get(WebSocket::new(get_quiz));
    app.listen("0.0.0.0:3001").await?;
    Ok(())
}

async fn get_quiz(req: Request<State>, mut stream: WebSocketConnection) -> tide::Result<()> {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let quiz = sqlx::query_as!(Quiz, "SELECT * FROM quizes WHERE qui_id = $1", quiz_id)
        .fetch_one(&req.state().pool)
        .await?;
    let questions = sqlx::query_as!(
        Question,
        "SELECT * FROM questions WHERE qui_id = $1",
        quiz_id
    )
    .fetch_all(&req.state().pool)
    .await?;

    stream
        .send_json(&WebsocketQuiz {
            message_type: WebsocketMessage::Quiz,
            name: quiz.name,
            description: quiz.description,
            num_questions: questions.len(),
        })
        .await?;
    

    let mut score = 0u32;

    for (idx, question) in questions.iter().enumerate() {
        let answers = sqlx::query_as!(
            Answer,
            "SELECT * FROM answers WHERE que_id = $1",
            question.que_id
        )
        .fetch_all(&req.state().pool)
        .await?;

        stream
            .send_json(&WebsocketQuestion {
                message_type: WebsocketMessage::Question,
                index: idx+1,
                text: question.que_text.clone(),
                image_url: question.image_url.clone(),
                alternatives: answers.iter().enumerate().map(|(i, a)| WebsocketAnswer {
                    index: i+1,
                    text: a.ans_text.clone()
                }).collect()
            })
            .await?;


        let correct_answers: Vec<usize> = answers
            .iter()
            .enumerate()
            .filter(|(_, ans)| match ans.correct {
                Some(correct) => correct,
                None => false,
            })
            .map(|(i, _)| i+1)
            .collect();

        // todo: use a loop to set submitted_answer once Message has arrived in the stream
        // or something more sensible than this
        let submitted_answer: IncomingWebsocketAnswer;

        loop {
            let ans: Option<IncomingWebsocketAnswer> = match stream.next().await {
                Some(Ok(Message::Text(input))) => Some(serde_json::from_str(&input)?),
                _ => None
            };
            if let Some(answer) = ans {
                submitted_answer = answer;
                break;
            }
        }

        if submitted_answer.answer == 0 || submitted_answer.answer > answers.len() || submitted_answer.index != idx+1 {
            stream
                .send_string("This is supposed to never happen".into())
                .await?;
            continue;
        }

        let correct_answer = correct_answers.contains(&submitted_answer.answer);
        if correct_answer {
            score += 1;
        }

        
        stream.send_json(&WebsocketResult {
            message_type: WebsocketMessage::Result,
            index: idx+1,
            correct: correct_answer,
            score: score,
            correct_answers: correct_answers,
        }).await?;
        
        thread::sleep(time::Duration::new(3, 0));
    }
    stream
        .send_json(&WebsocketEndResult {
            message_type: WebsocketMessage::End,
            score: score,
        })
        .await?;
    stream.send(Message::Close(None)).await?;
    Ok(())
}

async fn create_quiz(mut req: Request<State>) -> tide::Result {
    let quiz: IncomingQuiz = req.body_json().await?;

    let new_quiz = sqlx::query_as!(
        Quiz,
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

async fn create_question(mut req: Request<State>) -> tide::Result {
    let question: IncomingQuestion = req.body_json().await?;
    let quiz_id: i32 = req.param("q")?.parse()?;
    
    let new_question = sqlx::query_as!(
        Question,
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

async fn get_answers(req: Request<State>) -> tide::Result {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let quiz = sqlx::query_as!(
        Quiz,
        "SELECT * FROM quizes WHERE qui_id = $1",
        quiz_id
    )
    .fetch_one(&req.state().pool)
    .await?;

    let questions = sqlx::query_as!(
        Question,
        "SELECT * FROM questions WHERE qui_id = $1",
        quiz_id
    )
    .fetch_all(&req.state().pool)
    .await?;
    let mut internal_questions = Vec::new();
    for q in questions.iter() {
        let answers = sqlx::query_as!(
            Answer,
            "SELECT * FROM answers WHERE que_id = $1",
            q.que_id
        )
        .fetch_all(&req.state().pool)
        .await?;

        internal_questions.push(IncomingQuestion {
            que_text: q.que_text.clone(),
            image_url: q.image_url.clone(),
            answers: answers
            .iter()
            .map(|a| IncomingAnswer {
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