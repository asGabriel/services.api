FROM rust
WORKDIR /
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --workspace
COPY ./target/release/finance .
EXPOSE 8080
CMD ["finance"]