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

FROM chef AS planner

# кодярник
COPY ./auth/Cargo.toml ./auth/
COPY ./auth/src ./auth/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

RUN echo '[workspace]' > Cargo.toml && \
    echo 'members = ["auth", "utils"]' >> Cargo.toml && \
    echo 'resolver = "2"' >> Cargo.toml
# endкодярник

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release

# кодярник
COPY ./auth/Cargo.toml ./auth/
COPY ./auth/src ./auth/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

RUN echo '[workspace]' > Cargo.toml && \
    echo 'members = ["auth", "utils"]' >> Cargo.toml && \
    echo 'resolver = "2"' >> Cargo.toml
# endкодярник

RUN cargo build -r

FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/auth /app/auth
EXPOSE 80
ENTRYPOINT [ "/app/auth" ]