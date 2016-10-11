FROM alexcrichton/rust-slave-linux-cross:2016-04-15
USER root

RUN apt-get update

ENTRYPOINT ["sh"]

ENV PATH=$PATH:/rust/bin \
    CAN_CROSS=1 \
    CARGO_TARGET_X86_64_UNKNOWN_FREEBSD_LINKER=x86_64-unknown-freebsd10-gcc
