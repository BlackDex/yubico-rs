# Change to build a different example, like otp_async
ARG EXAMPLE=otp_async
# Use "-F blocking" if you want to test a blocking example
# Default example is async which does not need this
ARG FEATURES=

FROM rust:alpine AS base
ARG EXAMPLE
ARG FEATURES

RUN apk --no-cache add \
    gcc \
    g++ \
    openssl \
    openssl-dev \
    pkgconfig

COPY . /src

WORKDIR /src

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --release ${FEATURES} \
    --example "${EXAMPLE}"

FROM alpine:3
ARG EXAMPLE
RUN apk --no-cache add libgcc openssl

COPY --from=base "/src/target/release/examples/${EXAMPLE}" /usr/local/bin/otp

ENV RUST_BACKTRACE=1
ENTRYPOINT [ "/usr/local/bin/otp" ]
