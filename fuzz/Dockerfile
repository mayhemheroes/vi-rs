# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /vi-rs/

WORKDIR /vi-rs/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /vi-rs/fuzz/target/x86_64-unknown-linux-gnu/release/telex /

COPY --from=builder /vi-rs/fuzz/target/x86_64-unknown-linux-gnu/release/vni /

COPY --from=builder /vi-rs/fuzz/target/x86_64-unknown-linux-gnu/release/word_validation /

COPY --from=builder /vi-rs/fuzz/target/x86_64-unknown-linux-gnu/release/word_validation_arbitrary /