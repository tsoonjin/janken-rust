FROM rust:1.81 as builder
WORKDIR /usr/src/app
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml .
COPY shared shared
COPY frontend frontend
COPY backend backend
WORKDIR /usr/src/app/frontend
RUN trunk build

FROM nginx:alpine
COPY --from=builder /usr/src/app/frontend/dist /usr/share/nginx/html
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]