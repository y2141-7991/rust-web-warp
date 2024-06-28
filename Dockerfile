FROM rust:latest AS builder

RUN apt -y update
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app

COPY ./ .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'

RUN cargo build --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/rust-web-dev ./
COPY --from=builder /app/.env ./

CMD ["/app/rust-web-dev"]