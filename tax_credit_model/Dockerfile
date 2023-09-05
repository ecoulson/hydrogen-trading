FROM rust:latest as builder

WORKDIR /tmp/builder
COPY . .

ADD assets ./assets

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && \
    mv ./target/release/tax_credit_model_server ./tax_credit_model_server

# Runtime image
FROM debian:bookworm-slim

# Run as "tax_credit_model_prod" user
RUN useradd -ms /bin/bash tax_credit_model_prod

USER tax_credit_model_prod
WORKDIR /srv/tax_credit_model_server

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /tmp/builder/tax_credit_model_server /srv/tax_credit_model_server
COPY --from=builder /tmp/builder/assets /srv/tax_credit_model_server/assets

ENV ASSETS_DIRECTORY=/srv/tax_credit_model_server/assets
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Run the server
CMD ./tax_credit_model_server
