FROM debian:stretch

RUN echo "deb http://archive.debian.org/debian stretch main"                   > /etc/apt/sources.list
RUN echo "deb http://archive.debian.org/debian stretch-proposed-updates main" >> /etc/apt/sources.list
RUN echo "deb http://archive.debian.org/debian-security stretch/updates main" >> /etc/apt/sources.list

RUN apt-get update
RUN apt-get install -y curl git build-essential pkg-config libssl-dev systemd
RUN apt-get install -y libasound2-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.81 -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu

ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache

ADD . /src
WORKDIR /src
CMD ["/util/docker-build.sh"]