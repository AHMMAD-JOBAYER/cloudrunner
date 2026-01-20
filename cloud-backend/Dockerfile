# Build stage
FROM docker.io/rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Final stage
FROM gcr.io/distroless/cc-debian12
WORKDIR /
COPY --from=builder /app/target/release/cloud-backend /cloud-backend
COPY --from=builder /app/.env .env
USER nonroot:nonroot
EXPOSE 8080
CMD ["/cloud-backend"]
