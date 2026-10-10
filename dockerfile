FROM rust:latest
WORKDIR /usr/src/trading_engine
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/trading_engine"]