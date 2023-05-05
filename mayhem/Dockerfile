FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /vi-rs
WORKDIR /vi-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /vi-rs/fuzz/target/x86_64-unknown-linux-gnu/release/vi-fuzz /