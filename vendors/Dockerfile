FROM rust:latest
EXPOSE 4311

RUN apt-get update && apt-get install -y cmake

COPY ./vendors/Cargo.toml ./vendors/
COPY ./vendors/src ./vendors/src

COPY ./utils/Cargo.toml ./utils/
COPY ./utils/src ./utils/src

WORKDIR /vendors
RUN cargo build --release

CMD ["./target/release/vendors"]
