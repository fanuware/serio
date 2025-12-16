FROM debian:trixie-slim

RUN apt-get update && \
    apt-get -y install \
        build-essential \
        mingw-w64 \
        rustup \
        socat \
        util-linux && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup target add x86_64-pc-windows-gnu

COPY files/entrypoint.sh /usr/local/bin/entrypoint

WORKDIR /src

ENTRYPOINT ["/usr/local/bin/entrypoint"]
