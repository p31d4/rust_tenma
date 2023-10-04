FROM rust:latest

RUN apt update && \
        apt install -y libudev-dev

RUN cd /tmp && \
        git clone https://github.com/p31d4/rust_tenma.git && \
        cd rust_tenma && \
        cargo build --release
