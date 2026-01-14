set shell := ["nu", "-c"]

default:
    echo 'Hello, world!'

run:
    mprocs "npm run dev --prefix ./web" \
    "cargo run --bin server"
