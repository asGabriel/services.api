FROM rust
WORKDIR /
COPY . .
ARG SQLX_OFFLINE=true
ENV PORT=8080
RUN cargo build --release --workspace
COPY ./target/release/finance .
EXPOSE 8080
CMD ["./finance"]
