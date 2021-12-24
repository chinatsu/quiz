-- Add migration script here

CREATE TABLE sessions (
    session_id serial,
    quiz_id int NOT NULL,
    primary key (session_id),
    CONSTRAINT fk_quiz_id FOREIGN KEY (quiz_id) REFERENCES quizes(qui_id) ON DELETE CASCADE
);

CREATE TABLE players (
    player_id serial,
    session_id int NOT NULL,
    score int NOT NULL,
    primary key (player_id),
    CONSTRAINT fk_session_id FOREIGN KEY (session_id) REFERENCES sessions(session_id) ON DELETE CASCADE
)