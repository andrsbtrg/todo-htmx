FROM messense/rust-musl-cross:aarch64-musl as builder
ENV SQLX_OFFLINE=true
RUN echo $DATABASE_URL
WORKDIR /learn-htmx
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target aarch64-unknown-linux-musl
# Create a new stage with a minimal image

FROM scratch
COPY --from=builder /learn-htmx/target/aarch64-unknown-linux-musl/release/learn-htmx /learn-htmx
COPY --from=builder /learn-htmx/configuration/ /configuration/
COPY --from=builder /learn-htmx/templates /templates
COPY --from=builder /learn-htmx/static /static
ENV APP_ENVIRONMENT production
ENTRYPOINT ["/learn-htmx"]
EXPOSE 8000
