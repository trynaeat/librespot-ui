FROM debian:stretch

RUN echo "deb http://archive.debian.org/debian stretch main"                   > /etc/apt/sources.list
RUN echo "deb http://archive.debian.org/debian stretch-proposed-updates main" >> /etc/apt/sources.list
RUN echo "deb http://archive.debian.org/debian-security stretch/updates main" >> /etc/apt/sources.list

RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y curl git build-essential pkg-config libssl-dev systemd crossbuild-essential-armhf
RUN apt-get install -y libasound2-dev

# Install libssl for arm
RUN apt-get download libssl-dev:armhf
RUN mkdir /sysroot && \
    dpkg -x libssl-dev*.deb /sysroot/

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.81 -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add arm-unknown-linux-gnueabihf

RUN mkdir /.cargo && \
    echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"\nrustflags = ["-C", "link-args=-L/usr/arm-linux-gnueabihf/lib"]' >> /.cargo/config

ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig/
ENV C_INCLUDE_PATH=/sysroot/usr/include
ENV OPENSSL_LIB_DIR=/sysroot/usr/lib/arm-linux-gnueabihf
ENV OPENSSL_INCLUDE_DIR=/sysroot/usr/include/arm-linux-gnueabihf
ENV CMAKE_C_COMPILER=arm-linux-gnueabihf-gcc

ADD . /src
WORKDIR /src
CMD ["/util/docker-build.sh"]