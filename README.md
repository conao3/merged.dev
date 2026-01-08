# merged.dev

A full-stack web application with a Next.js frontend and Rust backend.

## Tech Stack

- **Frontend**: Next.js 14 (App Router), React 18, TypeScript
- **Backend**: Rust, Axum, SQLx
- **Database**: PostgreSQL 16
- **Infrastructure**: Docker Compose

## Project Structure

```
merged.dev/
├── frontend/           # Next.js application
│   ├── app/            # App Router pages and layouts
│   └── public/         # Static assets
├── backend/            # Rust API server
│   ├── src/            # Source code
│   │   ├── models/     # Database models
│   │   ├── db.rs       # Database connection
│   │   ├── lib.rs      # Library exports
│   │   └── main.rs     # Entry point
│   ├── tests/          # Integration tests
│   └── schema.sql      # Database schema (managed by sqldef)
└── compose.yaml        # Docker Compose for PostgreSQL
```

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://www.docker.com/) and Docker Compose

## Getting Started

### 1. Start the Database

```bash
docker compose up -d
```

This starts PostgreSQL on port 5432 with:

- User: `postgres`
- Password: `postgres`
- Database: `merged`

### 2. Apply Database Schema

Install [sqldef](https://github.com/sqldef/sqldef) and apply the schema:

```bash
psqldef -U postgres -W postgres -h localhost merged < backend/schema.sql
```

### 3. Run the Backend

```bash
cd backend
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/merged"
cargo run
```

The API server starts at `http://localhost:3000`.

### 4. Run the Frontend

```bash
cd frontend
pnpm install
pnpm dev
```

The frontend starts at `http://localhost:3001` (or the next available port).

## Development

### Backend Commands

```bash
cd backend
cargo build          # Build the project
cargo test           # Run tests
cargo clippy         # Run linter
```

### Frontend Commands

```bash
cd frontend
pnpm dev             # Start development server
pnpm build           # Build for production
pnpm lint            # Run ESLint
```
