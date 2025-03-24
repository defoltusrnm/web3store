FROM rust:latest

COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/keycloak_auth_rust"]
