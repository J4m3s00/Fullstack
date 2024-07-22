FROM rust:1.79 AS builder


WORKDIR /code/common
COPY ./../common/src ./src
COPY ./../common/Cargo.toml ./Cargo.toml


WORKDIR /code/backend
COPY . .


RUN cargo build --release


# Run the compiled binary

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /code/target/release/backend .

ENTRYPOINT ["./backend"]