# syntax=docker/dockerfile:1.4
FROM rust:1.80-slim-bullseye AS base
RUN apt update && apt install -y pkg-config libssl-dev
RUN rustup override set nightly

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base AS builder

RUN SQLX_OFFLINE=true cargo build --release

FROM debian:11.10-slim

WORKDIR /app
COPY --from=builder /code/target/release/satoshifamily /app/satoshifamily
COPY --from=builder /code/.sqlx /app/.sqlx
COPY --from=builder /code/assets /app/assets
COPY --from=builder /code/migrations /migrations
COPY --from=builder /code/src/templates /templates

CMD [ "./satoshifamily", "start"]   