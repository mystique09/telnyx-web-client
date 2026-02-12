# Telnyx Web Client

Web application for sending and receiving Telnyx messages, built as a Rust + React monorepo with Inertia.js.

## Tech Stack

- Backend: Rust (Edition 2024), Actix Web, Inertia integration (`actix-inertia`)
- Frontend: React 19, TypeScript 5.9, Vite (rolldown-vite), Tailwind CSS v4, shadcn/ui
- Data: PostgreSQL (`rbatis`, `rbdc-pg`, `bb8`)
- Architecture: Cargo workspace with DDD-style crate separation

## Prerequisites

- Rust toolchain (stable)
- Node.js (LTS) + npm
- PostgreSQL (or Docker for local Postgres)
- Optional dev tooling for `just run`: `just`, `nu`, `mprocs`

## Quick Start

1. Install frontend dependencies:
   ```bash
   npm install --prefix ./web
   ```
2. Create env file:
   ```bash
   copy .env.example .env
   ```
   On Unix systems:
   ```bash
   cp .env.example .env
   ```
3. Start local Postgres (optional if you already have one):
   ```bash
   docker compose up -d postgres
   ```
4. Run app:
   ```bash
   just run
   ```
   Alternative without `just`:
   ```bash
   cargo run --bin web-server
   npm run dev --prefix ./web
   ```
5. Open `http://127.0.0.1:8080`

## Environment Variables

Set these in `.env`:

| Name | Required | Description |
|---|---|---|
| `MODE` | Yes | `development` or `production` |
| `HOST` | Yes | Server bind host (example: `127.0.0.1`) |
| `PORT` | Yes | Server bind port (example: `8080`) |
| `PASETO_SEMETRIC_KEY` | Yes | Symmetric key for token service |
| `SESSION_SECRET` | Yes | Cookie session signing secret |
| `DATABASE_URL` | Yes | PostgreSQL connection URL |
| `VITE_ORIGIN` | No | Vite dev server origin (default `http://localhost:5173`) |
| `VITE_ENTRY` | No | Vite entry path for dev shell (recommended `/src/main.tsx`) |

Note: `PASETO_SEMETRIC_KEY` spelling must match exactly; the current code expects that exact key name.

## Common Commands

```bash
# Run backend
cargo run --bin web-server

# Check backend
cargo check

# Run backend tests
cargo test

# Build backend release
cargo build --release --bin web-server

# Run frontend dev server
npm run dev --prefix ./web

# Lint frontend
npm run lint --prefix ./web

# Build frontend
npm run build --prefix ./web
```

## Project Layout

```text
.
|- bin/web/src/main.rs
|- crates/
|  |- web/               # HTTP layer, handlers, middleware, Inertia response flow
|  |- application/       # Use cases
|  |- domain/            # Domain contracts/entities
|  |- infrastructure/    # DB/config/security implementations
|  `- workspace-hack/    # Shared dependency lock optimization (hakari)
|- web/
|  |- src/Pages/         # Inertia pages
|  |- src/features/      # Frontend feature modules
|  |- src/components/    # Shared UI + app components
|  `- vite.config.ts
|- .env.example
|- docker-compose.yml
`- justfile
```

## Production Build

```bash
npm run build --prefix ./web
cargo build --release --bin web-server
```

Build artifacts:
- Frontend: `web/dist`
- Backend binary: `target/release/web-server` (or `web-server.exe` on Windows)

## Notes

- Auth, conversations, and phone-number routes are implemented under `crates/web/src/handlers/`.
- Inertia page resolution is configured in `web/src/main.tsx`.
- Vite proxy routes (`/inertia`, `/inertia-version`) are configured in `web/vite.config.ts`.
