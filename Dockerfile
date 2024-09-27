FROM rust
WORKDIR /
COPY . .
RUN cargo build --release --workspace
COPY ./target/release/finance .
EXPOSE 8080
CMD ["finance"]