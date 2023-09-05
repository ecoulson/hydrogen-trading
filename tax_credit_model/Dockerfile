FROM rust:latest as builder

WORKDIR /usr/src/server
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/tax_credit_model ./tax_credit_model

# Runtime image
FROM debian:bookworm-slim

# Run as "server" user
RUN useradd -ms /bin/bash server

USER server
WORKDIR /server

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/server/tax_credit_model /server/tax_credit_model

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Run the server
CMD ./tax_credit_model
