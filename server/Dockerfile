FROM rust:latest

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
EXPOSE 8000

CMD ["./target/release/pylos"]