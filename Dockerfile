FROM debian:stable-slim
LABEL version="1.0.0" \
    authors="Yuri Becker <hi@yuri.li>"

RUN apt update && \
    apt install --yes --no-install-recommends \
      build-essential \
      ca-certificates \
      curl \
      chromium-common \
      chromium \
      chromium-l10n \
      chromium-driver \
      libssl-dev \
      pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup_init.sh
RUN chmod +x ./rustup_init.sh && ./rustup_init.sh -y
RUN . "$HOME/.cargo/env" && rustup toolchain install stable

COPY src /cc-logger/src
COPY .env /cc-logger/.env
COPY Cargo.lock /cc-logger/Cargo.lock
COPY Cargo.toml /cc-logger/Cargo.toml

RUN . "$HOME/.cargo/env" && cd /cc-logger && cargo build --release
RUN cp /cc-logger/target/release/cc-logger /usr/bin/cc-logger
ENV COMCAVE_NO_SANDBOX=true
ENV RUST_LOG=info

WORKDIR /cc-logger