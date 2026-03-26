FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV MODE=production \
    HOST=0.0.0.0 \
    PORT=8080

COPY server-bin/web-server /app/server
COPY web/dist /app/dist

RUN chmod +x /app/server

EXPOSE 8080

ENTRYPOINT ["/app/server"]
