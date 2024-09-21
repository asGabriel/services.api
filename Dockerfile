# Etapa 1: Compilação
FROM rust:1.72 as builder

# Defina o diretório de trabalho no contêiner
WORKDIR /usr/src/app

# Copie o Cargo.toml e lockfile do serviço específico
COPY services/finance/Cargo.toml Cargo.lock ./

# Copie o código-fonte do serviço específico
COPY services/finance ./services/finance/

# Compile o serviço 'finance'
RUN cargo build --release --package finance

# Etapa 2: Runtime
FROM debian:buster-slim

# Defina o diretório de trabalho no runtime
WORKDIR /usr/src/app

# Copie o binário compilado da etapa de build
COPY --from=builder /usr/src/app/target/release/finance .

# Exponha a porta onde o serviço vai rodar
EXPOSE 8080

# Comando para rodar o binário
CMD ["./finance"]
