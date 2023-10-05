FROM archlinux:latest

RUN echo "Y" | pacman -Syu

RUN echo "Y" | pacman -Sy vim git gcc rustup systemd-libs pkgconf

RUN rustup install stable
RUN rustup default stable

RUN cd /tmp && \
        git clone https://github.com/p31d4/rust_tenma.git && \
        cd rust_tenma && \
        cargo build --release
