FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
RUN echo $DATABASE_URL
WORKDIR /learn-htmx
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl
# Create a new stage with a minimal image

FROM scratch
COPY --from=builder /learn-htmx/target/x86_64-unknown-linux-musl/release/learn-htmx /learn-htmx
ENTRYPOINT ["/learn-htmx"]
EXPOSE 8000