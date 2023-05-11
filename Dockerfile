FROM rust:1.69 as builder
WORKDIR /home/akotro/apps/rubook/
COPY . .
RUN cargo build --bin rubook_backend --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev libmariadbclient-dev openssl && rm -rf /var/lib/apt/lists/*
RUN openssl req -x509 -newkey rsa:4096 -nodes -keyout /usr/local/bin/key.pem -out /usr/local/bin/certificate.pem -days 365 -subj "/CN=rubook"
COPY --from=builder /home/akotro/apps/rubook/target/release/rubook_backend /usr/local/bin/rubook_backend
WORKDIR /usr/local/bin/
CMD ["rubook_backend"]
