# Planner API

This is a simple task management API built with **Rust**, **Actix Web**, and **SeaORM**. The API uses a **PostgreSQL** database running inside a Docker container.

## Prerequisites

- **Docker**: Ensure you have Docker installed on your system.
- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Cargo**: Cargo is included with Rust, so no additional installation is needed.

## Steps to Run the Application

### 1. Start the PostgreSQL Database with Docker

First, ensure that Docker is running on your system. Then, navigate to the root directory of the project (where the `docker-compose.yml` file is located) and run:

```bash
docker-compose up -d
```
