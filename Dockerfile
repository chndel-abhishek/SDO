# Dockerfile
FROM arm32v7/debian:bookworm-slim

# Install build essentials + Rust
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        curl \
        gcc \
        pkg-config \
        libclang-dev \
        ca-certificates \
        --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build the app
RUN cargo build --release

# Default command: interactive shell
CMD ["/bin/bash"]