use tide::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Quiz {
    pub qui_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    pub ans_id: i32,
    pub ans_text: String,
    pub correct: Option<bool>,
    pub que_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub que_id: i32,
    pub que_text: String,
    pub image_url: Option<String>,
    pub qui_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlatQuiz {
    pub qui_id: i32,
    pub name: String,
    pub description: String,
    pub que_id: i32,
    pub que_text: String,
    pub image_url: Option<String>,
    pub ans_id: i32,
    pub ans_text: String,
    pub correct: Option<bool>,
}
