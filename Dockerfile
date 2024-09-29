FROM rust
WORKDIR /
COPY . .
ENV SQLX_OFFLINE=true
ENV DATABASE_URL=postgres://fictitious_user:fictitious_pass@localhost/fictitious_db
ENV PORT=8080
RUN cargo build --release --workspace
COPY ./target/release/finance .
EXPOSE 8080
CMD ["./finance"]
