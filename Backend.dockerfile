FROM rust:1.69.0-buster as builder

# Build out a dummy executable to update the cargo registry

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
WORKDIR /build

COPY ./.cargo /build/
COPY ./Cargo.toml /build/
COPY ./Cargo.lock /build/

COPY ./common /build/common/
COPY ./backend /build/backend/

RUN cargo build --release

WORKDIR /dist

RUN mv /build/target/release/aws_backend .
RUN mv /build/target/release/aws_migrator .

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /dist/aws_backend .
COPY --from=builder /dist/aws_migrator .

EXPOSE 3000

CMD ./aws_migrator && ./aws_backend
