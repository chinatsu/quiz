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

now the database should be set up with a stupid table. to start the server, do

```
cargo run
```

then curl or use some other tool to post a question

```
curl localhost:3000/questions/add -d '{"question": "Guess the right number", "answer": 1}'
```

have a look at `localhost:3000/questions/` to see your question, i guess.