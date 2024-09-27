FROM rustlang/rust:nightly-slim as builder

RUN apt-get update && apt-get install -y musl-tools --no-install-recommends

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

ENV PORT 8080
ENV FINANCE_PORT $PORT

WORKDIR /usr/bin

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/finance_service .

CMD ["finance_service"]
