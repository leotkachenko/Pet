# Rust + Postgres + Axum Pet Project

This is a small pet project developed using Rust, Postgres, and Axum.

## Overview
The project aims to showcase the integration of Rust, Postgres (a popular open-source relational database), and Axum (a lightweight web framework) for building web applications.

## Features

- **Rust:** The project is written in Rust, a safe and performant systems programming language known for its memory safety and concurrency features.
- **Postgres:** The project utilizes the Postgres database to store and manage data. Postgres is a robust and feature-rich database system.
- **Axum:** The project leverages Axum, a lightweight and ergonomic web framework, to handle HTTP requests, routing, and middleware.
- **Asynchronous Programming:** The project utilizes the Tokio async runtime to write asynchronous, non-blocking code for improved scalability and responsiveness.
- **SQLx:** SQLx, a Rust library, is used to interact with the Postgres database, providing a convenient and type-safe interface for executing SQL queries.

## Goal
The goal of this project is to provide an example and starting point for building Rust-based web applications that use Postgres as the backend database. It demonstrates the usage of Axum for handling HTTP requests and SQLx for interacting with the Postgres database.

## Usage
To use this project, follow these steps:

1. Clone the repository: `git clone https://github.com/your_username/your_project.git`
2. Install Rust and Cargo if you haven't already: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
3. Set up the Postgres database and configure the connection parameters in the project's configuration file.
4. Run the project: `cargo run` or `cargo watch -q -c -w src/ -x run` to debug
5. Access the web application at [http://localhost:8080](http://localhost:8080)

Feel free to explore, modify, and expand upon this project as per your requirements.

## License
This project is licensed under the [MIT License](LICENSE).
