FROM rust
WORKDIR /
COPY . .
RUN cargo install --path .
EXPOSE 8080
CMD ["finance"]