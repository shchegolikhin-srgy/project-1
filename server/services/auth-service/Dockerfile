FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 4002 5432

COPY --from=builder /usr/local/cargo/bin/auth-service /usr/local/bin/auth-service

CMD ["auth-service"]