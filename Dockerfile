FROM rust:1.67.0
RUN apt-get update && apt-get install -y openssl
WORKDIR /rpgt
COPY . .
RUN cargo build --release
ENTRYPOINT ["/rgpt/target/release/rgpt"]
