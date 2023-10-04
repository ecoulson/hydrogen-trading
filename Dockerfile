FROM rust:latest as builder

WORKDIR /tmp
COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cd tax_credit_model && \
    cargo build --release && \
    mv ./target/release/tax_credit_model_server ./tax_credit_model_server

# Runtime image
FROM debian:bookworm-slim

# Run as "tax_credit_model_prod" user
RUN useradd -ms /bin/bash tax_credit_model_prod

USER tax_credit_model_prod
WORKDIR /srv/tax_credit_model_server

ADD ./tax_credit_model/assets /srv/tax_credit_model_server/assets
ADD ./data /srv/tax_credit_model_server/data

RUN ls /srv/tax_credit_model_server

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /tmp/tax_credit_model/tax_credit_model_server /srv/tax_credit_model_server

RUN ls /srv/tax_credit_model_server

ENV ASSETS_DIRECTORY=/srv/tax_credit_model_server/assets
ENV DATA_DIRECTORY=/srv/tax_credit_model_server/data
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Run the server
CMD ./tax_credit_model_server
