FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
WORKDIR /rustapi

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /rust-rustapi/target/x86_64-unknown-linux-musl/release/rustapi /rustapi
ENTRYPOINT ["/rust-rustapi"]
EXPOSE 8080