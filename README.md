# Telnyx web client
A webapp to send and receive messages from Telnyx.

## Tech Stack

- **Backend**: Rust, Actix-web, Inertia.js
- **Frontend**: TypeScript, React, Vite, Inertia.js

## Project Structure

```
.
├── Cargo.toml              # Rust project config
├── justfile                # Task runner commands
├── src/                    # Rust source
│   └── main.rs             # Actix-web server
├── web/                    # Frontend Vite project
│   ├── src/
│   ├── dist/               # Built frontend assets
│   └── package.json
├── target/                 # Rust build output
```

## Commands

### Development

```bash
just run
```

Runs Vite dev server and Cargo dev server concurrently. Visit [http://localhost:8080](http://localhost:8080) to preview.

### Production Build

Build frontend and compile Rust binary:

```bash
npm run build --prefix ./web
cargo build --release
```

Move outputs to `output/` folder:

```bash
mkdir -p output
mv dist/ output/
mv target/release/server output/
```

Final structure:

```
output/
├── dist/           # Frontend assets
└── server          # Rust binary
```

Run the production server:

```bash
./output/server
```

## Environment Variables

- `MODE`: Set to `development` for dev mode (default)
- `VITE_ORIGIN`: Vite dev server URL (default: `http://localhost:5173`)
- `VITE_ENTRY`: Vite entry point (default: `/src/main.ts`)
