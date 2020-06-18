FROM rust:latest

WORKDIR /opt/app
RUN cargo install systemfd cargo-watch

#install dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "//aaa" > src/lib.rs \
    && cargo update \
    && rm -rf src

#build
COPY src src
RUN cargo build