# quiz

it's all kinda shoddy right now but, assuming [sqlx cli](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md) is installed, and npm on lts or something recent-ish

```
cd api
docker-compose up -d
sqlx database create
sqlx migrate run
cargo run
```

in another terminal

```
cd frontend
yarn install
yarn dev
```

should result in something running on :3000, e.g. [/quiz/0](http://localhost:3000/quiz/0)