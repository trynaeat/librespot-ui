#!/usr/bin/env bash
set -eux

 ln -s /usr/bin/arm-linux-gnueabihf-gcc /bin/arm-linux-gnueabihf-gcc
 ln -s /usr/bin/arm-linux-gnueabihf-cpp-8 /bin/arm-linux-gnueabihf-g++
PI1_LIB_DIRS=(
  "/sysroot/usr/lib/arm-linux-gnueabihf"
  "/usr/arm-linux-gnueabihf/lib"
)
export RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc ${PI1_LIB_DIRS[*]/#/-L}"

cargo build --release --target arm-unknown-linux-gnueabihf
