FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 4002

CMD ["./target/release/auth-service"]