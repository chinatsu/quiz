-- Add migration script here
INSERT INTO quizes (qui_id, name, description)
VALUES (0, 'Sample quiz', 'It has some questions');

INSERT INTO questions (que_id, que_text, qui_id)
VALUES 
    (0, 'What''s the world''s largest land mammal?', 0),
    (1, 'The Godfather was released in 1972; who played the title role?', 0),
    (2, 'Zn is the symbol of which chemical element?', 0),
    (3, 'What does a Geiger Counter measure?', 0);

INSERT INTO answers (ans_text, que_id, correct)
VALUES
    ('Elephant', 0, true),
    ('Blue Whale', 0, false),
    ('Rhino', 0, false),
    ('Giraffe', 0, false),
    ('Al Pacino', 1, false),
    ('Marlon Brando', 1, true),
    ('Robert De Niro', 1, false),
    ('Joe Pesci', 1, false),
    ('Silicon', 2, false),
    ('Scandium', 2, false),
    ('Zinc', 2, true),
    ('Caesium', 2, false),
    ('Alpha particles', 3, true),
    ('Beta particles', 3, true),
    ('Gamma rays', 3, true),
    ('Radiation', 3, true);
