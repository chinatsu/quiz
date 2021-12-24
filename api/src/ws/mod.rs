use crate::{State, db::FlatQuiz, api};
use async_std::prelude::*;
use std::{thread, time};
use tide::prelude::*;
use tide::Request;
use tide_websockets::{Message, WebSocketConnection};

#[derive(Debug, Deserialize, Serialize)]
enum WebsocketMessage {
    Quiz,
    Question,
    Result,
    End,
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

fn flat_to_nested_quiz(flat: Vec<FlatQuiz>) -> api::OutgoingQuiz {
    let mut quiz = api::OutgoingQuiz {
        name: flat[0].name.clone(),
        description: flat[0].description.clone(),
        qui_id: flat[0].qui_id,
        questions: Vec::new(),
    };

    let mut prev_question = &FlatQuiz{
        qui_id: 0,
        name: "".into(),
        description: "".into(),
        que_id: -1,
        que_text: "".into(),
        image_url: None,
        ans_id: 0,
        ans_text: "".into(),
        correct: None,
    };
    let mut ptr = 0usize;
    for line in flat.iter() {
        if prev_question.que_id != line.que_id {
            quiz.questions.push(api::NewQuestion {
                que_text: line.que_text.clone(),
                image_url: line.image_url.clone(),
                answers: Vec::new(),
            });
            ptr += 1;
        }
        quiz.questions[ptr-1].answers.push(api::NewAnswer {
            ans_text: line.ans_text.clone(),
            correct: line.correct,
        });
        prev_question = line;
    }
    quiz
}

pub async fn get_quiz(req: Request<State>, mut stream: WebSocketConnection) -> tide::Result<()> {
    let quiz_id: i32 = req.param("q")?.parse()?;
    let flat_quiz = sqlx::query_as!(
        FlatQuiz,
        r#"
        SELECT 
            a.qui_id, a.name, a.description, 
            b.que_id, b.que_text, b.image_url, 
            c.ans_id, c.ans_text, c.correct 
        FROM quizes a
        INNER JOIN questions b ON (b.qui_id = a.qui_id)
        INNER JOIN answers c ON (c.que_id = b.que_id)
        WHERE a.qui_id = $1
        "#,
        quiz_id
    )
        .fetch_all(&req.state().pool)
        .await?;
    let quiz = flat_to_nested_quiz(flat_quiz);

    stream
        .send_json(&WebsocketQuiz {
            message_type: WebsocketMessage::Quiz,
            name: quiz.name,
            description: quiz.description,
            num_questions: quiz.questions.len(),
        })
        .await?;
    

    let mut score = 0u32;

    for (idx, question) in quiz.questions.iter().enumerate() {
        stream
            .send_json(&WebsocketQuestion {
                message_type: WebsocketMessage::Question,
                index: idx+1,
                text: question.que_text.clone(),
                image_url: question.image_url.clone(),
                alternatives: question.answers.iter().enumerate().map(|(i, a)| WebsocketAnswer {
                    index: i+1,
                    text: a.ans_text.clone()
                }).collect()
            })
            .await?;


        let correct_answers: Vec<usize> = question.answers
            .iter()
            .enumerate()
            .filter(|(_, ans)| match ans.correct {
                Some(correct) => correct,
                None => false,
            })
            .map(|(i, _)| i+1)
            .collect();

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

        if submitted_answer.answer == 0 || submitted_answer.answer > question.answers.len() || submitted_answer.index != idx+1 {
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

