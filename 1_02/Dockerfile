FROM rust:latest

WORKDIR /usr/src/my_todo
COPY . .

ENV PORT=8080

RUN cargo install --path my_todo

CMD ["my_todo/target/release/my_todo"]

