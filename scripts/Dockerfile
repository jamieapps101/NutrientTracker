FROM ubuntu:jammy

ARG DOCKER_USER=default_user
RUN mkdir -p /home/$DOCKER_USER

RUN apt-get update
RUN apt-get install -y apt-utils
RUN apt-get upgrade -y
RUN apt-get install -y git curl build-essential
RUN apt-get install -y libssl-dev pkg-config

USER $DOCKER_USER
RUN mkdir /workspace

WORKDIR /workspace

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN chmod o+rx /root
ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_HOME="/workspace/.cargo"
ENV RUSTUP_HOME="/workspace/.rustup"
ENV PATH="$CARGO_HOME/bin:${PATH}"

# RUN cargo install sea-orm-cli

# this is here to initialise otherwise empty cargo/rustup caches
# RUN rustup default stable

CMD bash
