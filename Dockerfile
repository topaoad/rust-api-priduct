FROM rust:1.79-slim-bullseye as builder
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl1.1 \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-watch

# プロジェクトの依存関係をコピーしてビルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .
RUN cargo build --release
RUN ls -l target/release

# 2番目のステージ（最終的なイメージ）
FROM rust:1.79-slim-bullseye
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/cargo-watch /usr/local/bin/cargo-watch
COPY --from=builder /usr/src/app /usr/src/app

# Rustはコンパイラ言語だが、都度コンパイルするのは面倒なのでcargo-watchを使って自動コンパイルする
CMD ["cargo", "watch", "-x", "run"]