FROM rust:1.69.0-buster as builder

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN cargo install sccache

ENV HOME=/root
ENV SCCACHE_CACHE_SIZE="5G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

WORKDIR /build

COPY ./.cargo /build/
COPY ./Cargo.toml /build/
COPY ./Cargo.lock /build/

COPY ./common /build/common/
COPY ./backend /build/backend/

RUN --mount=type=cache,target=/root/.cache/sccache cargo build --release

WORKDIR /dist

RUN mv /build/target/release/aws_backend .
RUN mv /build/target/release/aws_migrator .

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /dist/aws_backend .
COPY --from=builder /dist/aws_migrator .

EXPOSE 3000

CMD ./aws_migrator && ./aws_backend
