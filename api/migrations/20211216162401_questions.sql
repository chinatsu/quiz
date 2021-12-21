-- Add migration script here
CREATE TABLE quizes (
    qui_id serial,
    name text NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (qui_id)
);

CREATE TABLE questions (
    que_id serial,
    que_text text NOT NULL,
    image_url text,
    qui_id int NOT NULL,
    PRIMARY KEY (que_id),
    CONSTRAINT fk_qui_id FOREIGN KEY (qui_id) REFERENCES quizes(qui_id) ON DELETE CASCADE
);

CREATE TABLE answers (
    ans_id serial,
    ans_text text NOT NULL,
    que_id int NOT NULL,
    correct boolean DEFAULT false,
    PRIMARY KEY (ans_id),
    CONSTRAINT fk_que_id FOREIGN KEY (que_id) REFERENCES questions(que_id) ON DELETE CASCADE
);