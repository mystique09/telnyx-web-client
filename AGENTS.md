# AGENTS.md - AI Agent Context for Telnyx Web Client

## Project Overview

**Project Name:** Telnyx Web Client  
**Purpose:** Web app to send and receive messages through Telnyx APIs  
**Architecture:** Full-stack monorepo with Rust backend and React frontend

## Tech Stack

### Backend (Rust workspace)
- Rust (Edition 2024)
- Actix Web 4.x
- Inertia integration via `actix-inertia`
- Tokio async runtime
- Telnyx API client via `reqwest`
- Session auth (`actix-session`, cookie store)
- Postgres access through `rbatis` + `rbdc-pg` + `bb8`
- Database migrations with `refinery`
- Logging with `tracing` + `color-eyre`
- Env loading with `dotenvy`
- Workspace dependency dedupe with `cargo-hakari` (`workspace-hack` crate)

### Frontend (React + TypeScript)
- React 19 + TypeScript 5.9
- Vite (rolldown-vite)
- Inertia.js (`@inertiajs/react`)
- Tailwind CSS v4
- shadcn/ui (New York, neutral base)
- react-hook-form + zod for form validation
- Lucide icons

## Repository Structure

```text
.
|- Cargo.toml
|- justfile
|- .env / .env.example
|- docker-compose.yml
|- bin/
|  |- web/
|  |  |- Cargo.toml
|  |  `- src/main.rs
|- crates/
|  |- web/
|  |  `- src/
|  |     |- handlers/
|  |     |  |- auth/
|  |     |  |- conversations/
|  |     |  |- events/
|  |     |  |- phone_numbers/
|  |     |  |- webhooks/
|  |     |  `- mod.rs
|  |     |- middlewares/
|  |     |- realtime.rs
|  |     |- server.rs
|  |     |- inertia.rs
|  |     |- session.rs
|  |     |- webhook_forwarding.rs
|  |     `- lib.rs
|  |- application/
|  |- domain/
|  |- infrastructure/
|  |  `- src/config/
|  |     |- web.rs
|  |     |- database.rs
|  |     `- mod.rs
|  |- telnyx/
|  `- workspace-hack/
`- web/
   |- package.json
   |- vite.config.ts
   |- components.json
   `- src/
      |- Pages/
      |- features/
      |- components/
      |- hooks/
      `- main.tsx
```

## Environment Variables

### Required backend variables (`.env`)
- `MODE`: `development` or `production`
- `HOST`: bind host (example: `127.0.0.1`)
- `PORT`: bind port (example: `8080`)
- `PASETO_SEMETRIC_KEY`: symmetric key used by the token service  
  Note: key name is intentionally spelled `SEMETRIC` in current code and must match.
- `SESSION_SECRET`: cookie session signing secret
- `TELNYX_API_KEY`: Telnyx API key used for outbound messaging
- `TELNYX_MESSAGING_PROFILE_ID`: Telnyx messaging profile used when sending messages
- `TELNYX_PUBLIC_KEY`: Telnyx webhook verification key
- `DATABASE_URL`: Postgres connection string

### Optional backend variables
- `VITE_ORIGIN`: Vite dev server URL (default: `http://localhost:5173`)
- `VITE_ENTRY`: Vite entry path for dev HTML shell (recommended: `/src/main.tsx`)
- `TELNYX_API_BASE_URL`: Telnyx API origin (default: `https://api.telnyx.com`)
- `TELNYX_WEBHOOK_FORWARD_URLS`: comma-separated list of additional webhook URLs that should receive every verified Telnyx messaging event

## Development Commands

### Recommended
```bash
just run
```

This runs Vite and Rust servers concurrently through `mprocs`.

### Manual
```bash
# backend
cargo run --bin web-server

# frontend
npm run dev --prefix ./web
```

### Build and checks
```bash
# backend
cargo check
cargo test
cargo build --release --bin web-server

# frontend
npm run lint --prefix ./web
npm run build --prefix ./web
```

### Local services (optional)
```bash
docker compose up -d postgres
```

Redis is also defined in `docker-compose.yml`, but current startup path only requires Postgres.

## Backend Architecture Notes

### Crate dependency map
- `bin/web` depends on `web` and `infrastructure`
- `crates/web` depends on `application` and `domain`
- `crates/application` depends on `domain` and `infrastructure`
- `crates/infrastructure` depends on `domain`
- `crates/telnyx` depends on `domain`
- all crates depend on `workspace-hack`

Treat this as the source of truth when deciding where to place code.

### HTTP routing (`crates/web/src/server.rs`)
- `/` -> dashboard page (`App`) with protected middleware
- `/auth/*` -> login/signup/password reset/logout handlers
- `/conversations` + `/conversations/{id}` -> conversation CRUD endpoints/pages
- `/conversations/{id}/messages` -> outbound message creation endpoint
- `/events/messages` -> protected SSE endpoint for realtime message updates
- `/phone-numbers` + `/phone-numbers/{id}` -> phone number CRUD endpoints
- `/webhooks/telnyx/messaging` -> public Telnyx messaging webhook endpoint
- `/version` -> Inertia version endpoint
- custom 404 handler renders Inertia `NotFound` page
- production mode serves static assets from `dist/`

### Middleware currently active
- `SessionMiddleware` (cookie session store)
- `NormalizePath::trim()`
- `Compress`
- `Logger`
- 404 error handler mapping to Inertia page
- route-level auth middleware (`GuestMiddleware`, `ProtectedMiddleware`)

### Startup flow (`bin/web/src/main.rs`)
1. Load tracing and `dotenv`.
2. Load `WebConfig` and `DatabaseConfig` from environment.
3. Create DB pool.
4. Run migrations (`migrator`).
5. Build repositories, the Telnyx client, realtime broadcaster, and webhook forwarder.
6. Start Actix server with graceful shutdown handling.

## Frontend Architecture Notes

### Frontend layout
- Inertia pages are in `web/src/Pages/`
- page-level feature composition lives in `web/src/features/`
  - `dashboard/`
  - `conversations/`
- reusable UI components are in `web/src/components/ui/`

### Current pages
- `App.tsx`
- `Conversations.tsx`
- `Login.tsx`
- `Signup.tsx`
- `ForgotPassword.tsx`
- `ResetPassword.tsx`
- `NotFound.tsx`

### Inertia setup (`web/src/main.tsx`)
- resolves pages with `import.meta.glob("./Pages/**/*.tsx", { eager: true })`
- default form `recentlySuccessfulDuration` set to `5000`
- prefetch cache set to `1m`, hover delay `150ms`
- adds a custom request header: `X-Custom-Header: value`

### Vite proxy (`web/vite.config.ts`)
- `/inertia` -> `http://127.0.0.1:8080`
- `/inertia-version` -> `http://127.0.0.1:8080`

### Path aliases
- `@/*` -> `web/src/*`

### Messaging Notes
- Outbound messages are sent through Telnyx and persisted locally before the client appends them.
- Telnyx delivery and inbound events are processed through `POST /webhooks/telnyx/messaging`.
- The conversations UI listens to `GET /events/messages` via `EventSource` for realtime updates.
- `TELNYX_WEBHOOK_FORWARD_URLS` forwards the original verified webhook body plus original Telnyx headers to additional downstream Telnyx-compatible webhook consumers.

## Where To Modify Code

| Task | Location |
|---|---|
| server wiring, routes, middleware | `crates/web/src/server.rs` |
| auth handlers | `crates/web/src/handlers/auth/` |
| conversation handlers | `crates/web/src/handlers/conversations/` |
| SSE handlers and realtime transport | `crates/web/src/handlers/events/`, `crates/web/src/realtime.rs` |
| phone number handlers | `crates/web/src/handlers/phone_numbers/` |
| Telnyx webhook handlers and forwarding | `crates/web/src/handlers/webhooks/`, `crates/web/src/webhook_forwarding.rs` |
| inertia HTML shell behavior | `crates/web/src/inertia.rs` |
| web config/env parsing | `crates/infrastructure/src/config/web.rs` |
| db config/env parsing | `crates/infrastructure/src/config/database.rs` |
| application use cases | `crates/application/src/` |
| domain models/contracts | `crates/domain/src/` |
| Telnyx HTTP client and webhook verification | `crates/telnyx/src/` |
| frontend pages | `web/src/Pages/` |
| frontend feature logic | `web/src/features/` |
| shared UI components | `web/src/components/` |

## Agent Guidelines For This Repo

1. Respect existing crate boundaries instead of placing everything in `crates/web`.
2. Keep type safety intact in both Rust and TypeScript; avoid `any` escapes.
3. Use Inertia navigation/data patterns instead of introducing ad-hoc client fetch flows unless explicitly needed.
4. When updating auth behavior, check both handler logic and middleware behavior.
5. Keep env var names exact to code expectations (including `PASETO_SEMETRIC_KEY` spelling).
6. For UI changes, stay consistent with existing shadcn/Tailwind style and neutral palette unless asked otherwise.
7. Before finishing, run relevant checks (`cargo check` and `npm run build --prefix ./web`) when execution permissions allow.
