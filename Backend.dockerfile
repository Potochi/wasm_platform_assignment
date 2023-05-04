FROM rust:1.69.0-buster as sscache

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN cargo install sccache

ENV SCCACHE_CACHE_SIZE="5G"
ENV SCCACHE_DIR=/cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

# ================== SSCACHE ====================

FROM sscache as builder

WORKDIR /build

COPY ./.cargo /build/
COPY ./Cargo.toml /build/
COPY ./Cargo.lock /build/

COPY ./common /build/common/
COPY ./backend /build/backend/

RUN --mount=type=cache,target=/cache/sccache cargo build --release

WORKDIR /dist

RUN mv /build/target/release/aws_backend .
RUN mv /build/target/release/aws_migrator .

FROM debian:buster

WORKDIR /app

COPY --from=builder /dist/aws_backend .
COPY --from=builder /dist/aws_migrator .

EXPOSE 3000

CMD ./aws_migrator && ./aws_backend
