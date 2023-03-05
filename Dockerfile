### Builder ###
FROM rust:1.67.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev openssl libssl-dev

WORKDIR /rgpt
COPY . .
RUN OPENSSL_STATIC=1 cargo build --target x86_64-unknown-linux-musl --release

### Runtime ###
FROM scratch
WORKDIR /rgpt
COPY --from=builder /rgpt/target/x86_64-unknown-linux-musl/release/rgpt .
CMD ["/rgpt/rgpt"]
 
