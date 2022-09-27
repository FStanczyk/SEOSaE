FROM rust:latest as build

WORKDIR /usr/simple-bidask
COPY ./ ./

RUN cargo install --path .

CMD ["simple-bidask"]
