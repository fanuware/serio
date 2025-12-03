FROM debian:trixie-slim

RUN apt-get update && \
    apt-get -y install \
        build-essential \
        mingw-w64 \
        rustup && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup target add x86_64-pc-windows-gnu

WORKDIR /src
