# rust-backend

A production-style Rust REST API built with **Axum + Tokio + MongoDB**.

## Stack

| Concern        | Crate(s)                              |
|----------------|---------------------------------------|
| Web framework  | axum, tower, tower-http               |
| Async runtime  | tokio                                 |
| Database       | mongodb, bson                         |
| Auth           | jsonwebtoken (JWT), argon2            |
| Serialisation  | serde, serde_json                     |
| Validation     | validator                             |
| Error handling | thiserror                             |
| Config         | dotenvy                               |
| Tracing        | tracing, tracing-subscriber           |

## Project Structure

```
src/
├── main.rs               # bootstrap: config → DB → router → serve
├── config.rs             # env-driven Config struct
├── db.rs                 # MongoDB client init
├── state.rs              # AppState { db, config }
├── error.rs              # AppError enum → JSON IntoResponse
├── routes/mod.rs         # central router (nests all /api/* sub-routers)
├── middleware/
│   ├── auth.rs           # JWT bearer middleware
│   ├── rbac.rs           # role guard (Admin / User)
│   └── file_interceptor.rs  # multipart validation + disk streaming
└── modules/
    ├── auth/             # POST /api/auth/register, login; GET /api/auth/me
    ├── user/             # CRUD /api/users  (admin delete)
    └── media/            # POST /api/media/upload; GET /api/media; DELETE /api/media/:id
uploads/                  # media storage (gitignored, served at /uploads)
```

## Quick Start

### Prerequisites

- Rust (stable) — `rustup update stable`
- A running MongoDB instance (`mongodb://localhost:27017` by default)

### 1. Configure

```bash
cp .env.example .env
# Edit .env — set MONGO_URI and a strong JWT_SECRET
```

### 2. Run

```bash
./run.sh          # one-click: creates .env if missing, ensures uploads/, then cargo run
# or
make run          # cargo run
make dev          # cargo watch -x run  (requires cargo-watch)
make build        # cargo build --release
make all          # fmt + clippy + build
```

### 3. Verify

```bash
curl http://localhost:8080/health
# {"status":"ok","version":"0.1.0"}
```

## API Reference

### Auth

| Method | Path                  | Auth     | Description        |
|--------|-----------------------|----------|--------------------|
| POST   | /api/auth/register    | Public   | Register new user  |
| POST   | /api/auth/login       | Public   | Login, get JWT     |
| GET    | /api/auth/me          | Bearer   | Current user info  |

### Users

| Method | Path              | Auth           | Description         |
|--------|-------------------|----------------|---------------------|
| GET    | /api/users        | Admin          | List all users      |
| GET    | /api/users/:id    | Bearer         | Get user by id      |
| PATCH  | /api/users/:id    | Bearer (owner) | Update own profile  |
| DELETE | /api/users/:id    | Admin          | Delete user         |

### Media

| Method | Path                  | Auth   | Description                    |
|--------|-----------------------|--------|--------------------------------|
| POST   | /api/media/upload     | Bearer | Upload file (multipart)        |
| GET    | /api/media            | Bearer | List own files                 |
| DELETE | /api/media/:id        | Bearer | Delete own file                |

Static files are served at `/uploads/<filename>`.

## JWT

Tokens expire after **1 hour**. Include in requests as:

```
Authorization: Bearer <token>
```

## Environment Variables

| Variable       | Default             | Description                    |
|----------------|---------------------|--------------------------------|
| PORT           | 8080                | Listen port                    |
| MONGO_URI      | *(required)*        | MongoDB connection string      |
| DB_NAME        | rust_backend        | Database name                  |
| JWT_SECRET     | *(required)*        | HMAC-SHA256 signing key        |
| UPLOAD_DIR     | ./uploads           | File storage directory         |
| MAX_UPLOAD_MB  | 10                  | Maximum upload size (MiB)      |
| RUST_LOG       | info                | Log filter                     |
