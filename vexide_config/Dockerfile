# Use the official Rust image as a base
FROM rust:latest

# Install necessary packages
RUN apt-get update && apt-get install -y \
    udev \
    libudev-dev \
    build-essential \
    curl \
    && apt-get clean \
    && rustup override set nightly \
    && rustup component add rust-src \
    && cargo install cargo-v5 \
    && cargo install cargo-binutils \
    && rustup component add llvm-tools-preview

# Create a working directory inside the container
WORKDIR /mnt/

# Optionally, copy your project files into the container

# Expose any ports if needed (uncomment if needed)
# EXPOSE 8080

# Command to run when the container starts (you can override this)
CMD ["bash"]

