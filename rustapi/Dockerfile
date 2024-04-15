FROM rust:1.76 AS builder

COPY Cargo.lock Cargo.toml ./
COPY src ./src
ENV DATABASE_URL="postgresql://postgres:dbe900447032dd1d951acf9df653c954c98a86717754de12b253328ba61d5752@localhost:5432"
ENV CLIENT_ID="50375690088-d550pisn0njk98emapjq169fcre5iqp8.apps.googleusercontent.com"
ENV CLIENT_SECRET="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"
ENV JWT_SECRET="dee369bec579dcaaec2a1fabaf0d513c96ceae651396e1f4da9251839ea74fea"
ENV SERVER_HOST ="127.0.0.1"
ENV SERVER_PORT ="8080"

RUN cargo build --release

FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y ca-certificates libssl3

COPY --from=builder ./target/release/rustapi ./target/release/rustapi

EXPOSE 8080

CMD ["/target/release/rustapi"]