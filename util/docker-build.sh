#!/usr/bin/env bash
set -eux

cargo build --release --no-default-features --features alsa-backend
