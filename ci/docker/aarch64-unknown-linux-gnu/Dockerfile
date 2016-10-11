FROM ubuntu:16.10
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc-aarch64-linux-gnu libc6-dev-arm64-cross
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    PATH=$PATH:/rust/bin
