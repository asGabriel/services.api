FROM debian:latest

WORKDIR /app

COPY target/release/finance ./finance
COPY target/release/bff ./bff

EXPOSE 8080
EXPOSE 8081

CMD ["sh", "-c", "./finance & ./bff"]
