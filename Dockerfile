FROM debian:latest

RUN apt-get update && apt-get install -y \
ca-certificates \
&& rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get install -y postgresql-client && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/finance ./finance

EXPOSE 8080

CMD ./cloud-sql-proxy $GCP_DB_CONNECTION -p 5432 & ./finance
