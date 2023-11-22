FROM ubuntu:22.04 AS builder
RUN apt-get update \
    && apt-get install -y curl apt-utils build-essential
RUN curl -L -o caddy.deb "https://github.com/caddyserver/caddy/releases/download/v2.7.5/caddy_2.7.5_linux_amd64.deb"
RUN apt-get install -y ./caddy.deb


# Install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
RUN cargo build
# RUN /app/target/debug/temp-download-test &
# ENTRYPOINT ["/usr/bin/caddy", "run", "--config", "/app/Caddyfile"]
ENTRYPOINT ["/app/start-caddy-and-server.sh"]