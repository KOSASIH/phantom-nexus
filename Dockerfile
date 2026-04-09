FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release --bin nexus-node

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nexus-node /usr/local/bin/

EXPOSE 8545 8546
ENTRYPOINT ["nexus-node"]
CMD ["--testnet"]
