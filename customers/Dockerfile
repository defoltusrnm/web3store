FROM rust:latest
EXPOSE 4311

RUN apt-get update && apt-get install -y cmake

COPY ./customers/Cargo.toml ./customers/
COPY ./customers/src ./customers/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

WORKDIR /customers
RUN cargo build --release

CMD ["./target/release/customers"]
