FROM rust:latest
EXPOSE 4310

RUN apt-get update && apt-get install -y cmake

COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/keycloak_auth_rust"]
