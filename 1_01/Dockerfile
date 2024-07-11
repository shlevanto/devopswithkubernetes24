FROM rust:latest

WORKDIR /usr/src/getting_started
COPY . .

RUN cargo install --path getting_started

CMD ["getting_started/target/release/getting_started"]

