# NOTED

This is a Markdown note-taking app, with it's back-end written in Rust. Part of the [Coding Challenges](https://codingchallenges.substack.com/p/coding-challenge-35-google-keep)

## How to run

Run `cargo run` at the project's root folder. Optionally, create an `.env` file with this values:

```sh
# .env

PORT_NUMBER= 3000
```

## Stack

### Back-end

- Rust (with Axum)
- PostgreSQL

## Front-end

- Svelte (in a separate [repo](https://github.com/amrllkmn/noted-front-end))
- TailwindCSS
