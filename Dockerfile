FROM rust:latest as build

WORKDIR /usr/simple-bidask
COPY ./ ./

RUN cargo build --release

CMD ["./target/release/simple-bidask"]