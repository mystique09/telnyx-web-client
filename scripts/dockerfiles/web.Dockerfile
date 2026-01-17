FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY web-bin/web-server /app/wev-server
RUN chmod +x /app/web-server

COPY .env.example /app/.env

EXPOSE 8080

ENTRYPOINT ["/app/web-server"]
