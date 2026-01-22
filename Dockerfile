FROM node:22-alpine AS web-builder
WORKDIR /app/web
COPY web/package*.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

FROM rust:alpine AS rust-builder
RUN apk add --no-cache musl-dev sqlite-dev
WORKDIR /app
COPY Cargo.toml ./
COPY entity ./entity
COPY migration ./migration
COPY server ./server
COPY --from=web-builder /app/server/static ./server/static
ENV SKIP_WEB_BUILD=1
RUN cargo build --release -p server

FROM alpine:latest
RUN apk add --no-cache sqlite-libs ca-certificates
WORKDIR /app
COPY --from=rust-builder /app/target/release/server ./server
COPY --from=web-builder /app/server/static ./static
RUN mkdir -p /app/data
ENV DATABASE_URL=sqlite:./data/plain_word.db?mode=rwc
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
EXPOSE 3000
VOLUME ["/app/data"]
CMD ["./server"]