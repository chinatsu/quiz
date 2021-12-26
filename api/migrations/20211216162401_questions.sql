-- Add migration script here
CREATE TABLE quizes (
    quiz_id serial,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (quiz_id)
);

CREATE TABLE questions (
    question_id serial,
    question_text text NOT NULL,
    image_url text,
    quiz_id int NOT NULL,
    PRIMARY KEY (question_id),
    CONSTRAINT fk_quiz_id FOREIGN KEY (quiz_id) REFERENCES quizes(quiz_id) ON DELETE CASCADE
);

CREATE TABLE answers (
    answer_id serial,
    answer_text text NOT NULL,
    question_id int NOT NULL,
    correct boolean DEFAULT false,
    PRIMARY KEY (answer_id),
    CONSTRAINT fk_question_id FOREIGN KEY (question_id) REFERENCES questions(question_id) ON DELETE CASCADE
);