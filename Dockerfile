FROM rust:1.62

WORKDIR /app
COPY . .

RUN cargo install cargo-watch