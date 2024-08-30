FROM rust:1.80.1-bullseye

RUN apt update && apt upgrade -y
RUN apt update --fix-missing && apt install -y sudo wget curl vim git build-essential

# Add custom user "dev" with sudo permissions
RUN useradd dev -u 1000 -m -s /bin/bash && \
	echo "dev ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
USER dev

# Install cargo-binstall use cargo binstall for install prebuilt binaries
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Command runner similar to make
RUN cargo binstall just --no-confirm

# Terminal prompt customizations
COPY custom/gu /usr/local/bin/gu
COPY custom/.bashrc /home/dev/.bashrc
RUN sudo chown dev:dev /usr/local/bin/gu
RUN sudo chown dev:dev /home/dev/.bashrc