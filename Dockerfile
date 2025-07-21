FROM rust:1.86-alpine AS builder

RUN apk add --no-cache \
    curl \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p imphnen-backend/src imphnen-cms/src imphnen-dimentorin/src \
    imphnen-entities/src imphnen-gacha/src imphnen-gateway/src \
    imphnen-iam/src imphnen-libs/src imphnen-middleware/src \
    imphnen-utils/src tests/src && \
    echo "fn main() {}" > imphnen-backend/src/main.rs && \
    find . -name "src" -type d -exec sh -c 'echo "// dummy" > "$1/lib.rs"' _ {} \;

RUN echo '[package]\nname = "tests"\nversion = "0.1.0"\nedition = "2021"' > tests/Cargo.toml


RUN echo -e '[package]\nname = "tests"\nversion = "0.1.0"\nedition = "2021"' > tests/Cargo.toml


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

RUN RUSTFLAGS="-C target-cpu=generic -C opt-level=s -C panic=abort -C codegen-units=1 -C strip=symbols" \
    cargo build -p imphnen-backend --release && \
    strip target/release/api && \
    upx --best --lzma target/release/api 2>/dev/null || true

FROM scratch AS runner
COPY --from=builder /app/target/release/api /api
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENTRYPOINT ["/api"]