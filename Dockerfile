FROM rust:latest

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY .env .env

COPY services/finance services/finance
COPY services/bff services/bff
COPY clients clients
COPY libs libs

RUN cargo build --release --workspace

EXPOSE 8080
EXPOSE 8081

CMD ["sh", "-c", "./target/release/finance & ./target/release/bff"]
