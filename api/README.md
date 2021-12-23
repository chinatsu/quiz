# quiz api

idk if i'll actually work this out, but whatever.

## requirements

to create the database and running migrations, we need sqlx-cli.
this assumes that `~/.cargo/bin` is in your PATH.
```
cargo install sqlx-cli
```

also, docker to simplify setting up the postgres instance


## setup

```
docker-compose up -d
sqlx database create # this is based on our connection string in .env
sqlx migrate run
```

now the database should be set up with some tables and a sample quiz on id 0. to start the server, do

```
cargo run
```

to create your own quiz, use curl or some other tool to perform post requests

```
curl localhost:3000/create/quiz -d '{"name": "my quiz", "description": "it is my quiz!"}'
```

this will return a json object with a qui_id-field. you can use this in requests to create questions,
and also to play your quiz later.


to create a question, here with multiple correct answers because why not
```
curl localhost:3000/create/quiz/1/question -d '{"que_text": "Moonshine was not a slang term for which type of beverage?", "answers": [{"ans_text": "Alcohol"}, {"ans_text": "Juice", "correct": true}, {"ans_text": "Milk", "correct": true}, {"ans_text": "Water", "correct": true}]}'
```

note that the endpoint uses the qui_id-field, your first quiz will likely have an id of 1.

once you've created a few questions, you can play the quiz with websocat or something similar to interact with websockets.

```
websocat -E ws://localhost:3000/quiz/1
```