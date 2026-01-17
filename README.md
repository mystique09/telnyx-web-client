# Telnyx web client
A webapp to send and receive messages from Telnyx.

## Tech Stack

- **Backend**: Rust (2024 edition), Actix-web, Inertia.js
- **Frontend**: TypeScript, React 19, Vite, Inertia.js
- **Architecture**: Workspace with domain-driven design (DDD) layers

## Project Structure

```
.
├── Cargo.toml              # Workspace configuration
├── justfile                # Task runner commands
├── .config/
│   └── hakari.toml         # Cargo hakari config for dependency management
├── bin/
│   └── web/                # Binary entry point
│       ├── Cargo.toml
│       └── src/
│           └── main.rs     # Actix-web server entry point
├── crates/                 # Workspace crates
│   ├── web/                # HTTP layer (handlers, server, inertia)
│   │   ├── src/
│   │   │   ├── handlers/   # Request handlers (auth, inertia)
│   │   │   ├── server.rs   # Web service creation
│   │   │   ├── inertia.rs  # Inertia.js integration
│   │   │   └── types.rs    # Type definitions
│   │   └── Cargo.toml
│   ├── infrastructure/     # Infrastructure layer (config, external deps)
│   │   ├── src/
│   │   │   └── config/     # Configuration management
│   │   └── Cargo.toml
│   ├── domain/             # Domain layer (business logic, entities)
│   │   └── Cargo.toml
│   ├── application/        # Application layer (use cases, orchestration)
│   │   └── Cargo.toml
│   └── workspace-hack/     # Centralized dependency management
├── web/                    # Frontend Vite project
│   ├── src/
│   ├── dist/               # Built frontend assets
│   └── package.json
└── target/                 # Rust build output
```

## Commands

### Development

```bash
just run
```

Runs Vite dev server and Cargo dev server concurrently. Visit [http://localhost:8080](http://localhost:8080) to preview.

### Manual Development

```bash
# Backend (run workspace binary)
cargo run --bin web-server

# Frontend
cd web && npm run dev
```

### Production Build

Build frontend and compile Rust binary:

```bash
npm run build --prefix ./web
cargo build --release --bin web-server
```

Move outputs to `output/` folder:

```bash
mkdir -p output
mv web/dist output/
mv target/release/web-server output/
```

Final structure:

```
output/
├── dist/           # Frontend assets
└── web-server       # Rust binary
```

Run the production server:

```bash
./output/web-server
```

### Workspace Commands

```bash
# Run workspace-specific binary
cargo run --bin web-server

# Build specific crate
cargo build -p web

# Run tests across workspace
cargo test

# Check workspace dependencies (hakari)
cargo hakari generate
```

## Environment Variables

- `MODE`: Set to `development` for dev mode (default: production)
- `VITE_ORIGIN`: Vite dev server URL (default: `http://localhost:5173`)
- `VITE_ENTRY`: Vite entry point (default: `/src/main.ts`)
- Server runs on port 8080

## Architecture

### Workspace Layout

The project follows a domain-driven design (DDD) approach with clear separation of concerns:

- **bin/web/** - Application binary and entry point
- **crates/web/** - HTTP layer: request handlers, server setup, Inertia.js integration
- **crates/infrastructure/** - Infrastructure: configuration, external services, adapters
- **crates/domain/** - Domain: business entities, value objects, domain logic
- **crates/application/** - Application: use cases, orchestration between layers
- **crates/workspace-hack/** - Centralized dependency management via cargo-hakari

### Dependency Flow

```
bin/web (application)
    ↓
crates/web (HTTP)
    ↓
crates/application (use cases)
    ↓
crates/domain (business logic)
    ↓
crates/infrastructure (config, external deps)
```

All crates depend on `workspace-hack` for efficient dependency management.
