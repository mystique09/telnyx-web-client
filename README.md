# Telnyx Web Client

Web application for sending and receiving Telnyx messages, built as a Rust + React monorepo with Inertia.js. The current app supports outbound SMS through Telnyx, verified Telnyx messaging webhooks, realtime message updates over SSE, and optional forwarding of verified webhook events to additional Telnyx-compatible endpoints.

## Tech Stack

- Backend: Rust (Edition 2024), Actix Web, Inertia integration (`actix-inertia`)
- Telnyx integration: `reqwest`, Ed25519 webhook verification
- Frontend: React 19, TypeScript 5.9, Vite (rolldown-vite), Tailwind CSS v4, shadcn/ui
- Data: PostgreSQL (`rbatis`, `rbdc-pg`, `bb8`)
- Architecture: Cargo workspace with DDD-style crate separation

## Current Features

- Session-based authentication
- Phone number management
- Conversation creation with recipient phone numbers
- Outbound Telnyx message sending from `/conversations/{id}/messages`
- Telnyx messaging webhook processing at `/webhooks/telnyx/messaging`
- Realtime conversation updates through `GET /events/messages`
- Optional forwarding of verified Telnyx webhook events to additional webhook endpoints

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
| `TELNYX_API_KEY` | Yes | Telnyx API key used for outbound messaging |
| `TELNYX_MESSAGING_PROFILE_ID` | Yes | Messaging profile ID used in Telnyx send requests |
| `TELNYX_PUBLIC_KEY` | Yes | Telnyx public key used to verify webhook signatures |
| `DATABASE_URL` | Yes | PostgreSQL connection URL |
| `VITE_ORIGIN` | No | Vite dev server origin (default `http://localhost:5173`) |
| `VITE_ENTRY` | No | Vite entry path for dev shell (recommended `/src/main.tsx`) |
| `TELNYX_API_BASE_URL` | No | Telnyx API base URL (default `https://api.telnyx.com`) |
| `TELNYX_WEBHOOK_FORWARD_URLS` | No | Comma-separated list of additional webhook URLs that should receive every verified Telnyx messaging event |

Note: `PASETO_SEMETRIC_KEY` spelling must match exactly; the current code expects that exact key name.

Example forwarding config:

```env
TELNYX_WEBHOOK_FORWARD_URLS=https://example.com/webhooks/telnyx,https://internal.example.net/telnyx/messaging
```

Forwarded webhook requests preserve:
- the original raw JSON body
- the original incoming Telnyx headers, including `telnyx-signature-ed25519` and `telnyx-timestamp`

Only transport-managed headers such as `host`, `content-length`, and `connection` are excluded.

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
|  |- telnyx/            # Telnyx API client + webhook verification
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
- Realtime message streaming is implemented under `crates/web/src/handlers/events/` and `crates/web/src/realtime.rs`.
- Telnyx webhook forwarding is implemented in `crates/web/src/webhook_forwarding.rs`.
- Inertia page resolution is configured in `web/src/main.tsx`.
- Vite proxy routes (`/inertia`, `/inertia-version`) are configured in `web/vite.config.ts`.
