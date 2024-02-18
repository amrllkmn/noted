# NOTED

This is a Markdown note-taking app, with it's back-end written in Rust. Part of the [Coding Challenges](https://codingchallenges.substack.com/p/coding-challenge-35-google-keep)

## How to run

Run `cargo run` at the project's root folder. Optionally, create an `.env` file with this values:

```sh
# .env

PORT_NUMBER= 3000
DATABASE_URL= YOUR_DATABASE_URL
```

OR

Use docker-compose, which compiles the app and builds up a PostgreSQL DB if you want to skip the .env setup but remember to run `docker-compose up --build` if you have changes (I didn't set up hot reload).

## Stack

### Back-end

- Rust (with Axum)
- PostgreSQL

## Front-end

- Svelte (in a separate [repo](https://github.com/amrllkmn/noted-front-end))
