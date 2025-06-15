FROM rust:latest

WORKDIR /app


COPY . .

RUN cargo build --release

EXPOSE 4002

# Запускаем приложение
CMD ["./target/release/auth-service"]