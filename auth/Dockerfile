FROM rust:latest
EXPOSE 80

RUN apt-get update && apt-get install -y cmake

COPY ./auth/Cargo.toml ./auth/
COPY ./auth/src ./auth/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

WORKDIR /auth
RUN cargo build --release

CMD ["./target/release/auth"]
