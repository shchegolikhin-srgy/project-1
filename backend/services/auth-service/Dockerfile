FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app

CMD ["app"]