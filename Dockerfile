###############################################################################
## Builder
###############################################################################
FROM rust:1.64 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

# Create appuser
ENV USER=app
ENV UID=10001

ARG TARGET=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGET
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

RUN rustup target add $TARGET && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        libssl-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target $TARGET && \
    cp /app/target/$TARGET/release/simpleserver /app/simpleserver

###############################################################################
## Final image
###############################################################################
FROM alpine:3.17

# Set the work dir
WORKDIR /app

COPY content /app/content

# Copy our build
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/simpleserver /app/

RUN mkdir -p /app/db /app/audios && \
    chown -R app: /app

USER app

CMD ["/app/simpleserver"]
