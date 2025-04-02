FROM rust:alpine AS chef
WORKDIR /app
RUN apk --no-cache add \
    bash \
    g++ \
    ca-certificates \
    lz4-dev \
    musl-dev \
    cyrus-sasl-dev \
    openssl-dev \
    make \
    cmake \
    python3 \
    openssl-libs-static
RUN cargo install --locked cargo-chef
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

FROM chef AS planner

COPY ./customers/Cargo.toml ./customers/
COPY ./customers/src ./customers/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

RUN echo '[workspace]' > Cargo.toml && \
    echo 'members = ["customers", "utils"]' >> Cargo.toml && \
    echo 'resolver = "2"' >> Cargo.toml

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release

COPY ./customers/Cargo.toml ./customers/
COPY ./customers/src ./customers/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

RUN echo '[workspace]' > Cargo.toml && \
    echo 'members = ["customers", "utils"]' >> Cargo.toml && \
    echo 'resolver = "2"' >> Cargo.toml

RUN cargo build -r --verbose

FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/customers /app/customers
ENTRYPOINT [ "/app/customers" ]