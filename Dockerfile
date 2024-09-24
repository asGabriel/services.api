FROM rust

WORKDIR /

COPY Cargo.toml .
COPY Cargo.lock .

COPY services/finance services/finance
COPY services/bff services/bff

## Finance Build
WORKDIR /services/finance
RUN cargo install --path .

## Bff Build
WORKDIR /services/bff
RUN cargo install --path .

EXPOSE 8080
EXPOSE 8081

CMD ["finance", "bff"]