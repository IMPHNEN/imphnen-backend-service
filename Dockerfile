FROM rust:1.85-slim-bullseye AS builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY imphnen-backend ./imphnen-backend
COPY imphnen-cms ./imphnen-cms
COPY imphnen-dimentorin ./imphnen-dimentorin
COPY imphnen-entities ./imphnen-entities
COPY imphnen-gacha ./imphnen-gacha
COPY imphnen-gateway ./imphnen-gateway
COPY imphnen-iam ./imphnen-iam
COPY imphnen-libs ./imphnen-libs
COPY imphnen-middleware ./imphnen-middleware
COPY imphnen-utils ./imphnen-utils
COPY tests ./tests

RUN cargo fetch

RUN cargo build -p imphnen-backend --release \
  && strip target/release/api

FROM gcr.io/distroless/cc AS runner

WORKDIR /app

COPY --from=builder /app/target/release/api .

CMD ["/app/api"]
