FROM rust:1.81 as builder
WORKDIR /usr/src/app
# Copy workspace files
COPY Cargo.toml .
COPY shared shared
COPY frontend frontend
COPY backend backend
# Build with optimizations for size
ENV CARGO_PROFILE_RELEASE_OPT_LEVEL=1
ENV CARGO_PROFILE_RELEASE_LTO=true
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/backend .
RUN apt-get update && \
    apt-get install -y libpq5 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
EXPOSE 8080
CMD ["./backend"]