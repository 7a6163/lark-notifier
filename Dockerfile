FROM rust:1.85-slim AS builder
WORKDIR /usr/src/lark-notifier

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

ENV LARK_WEBHOOK_URL ''
ENV LARK_SECRET ''

COPY --from=builder /usr/src/lark-notifier/target/release/lark-notifier /lark-notifier

ENTRYPOINT ["./lark-notifier"]
