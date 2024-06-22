# Use the official Rust image as the base image
FROM rust:latest

# Install LLVM and Clang
RUN apt-get update && apt-get install -y  \
    llvm-14*  \
    clang-14  \
    llvm-14-dev \
    libclang-14-dev \
    libpolly-14-dev

ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config

ENV LD_LIBRARY_PATH="/usr/lib/llvm-14/lib:$LD_LIBRARY_PATH"

# Create a new directory for your project
WORKDIR /usr/src/love-js

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build your Rust project inside the container
RUN cargo build --release

# Run the executable by default when the container starts
CMD ["cargo", "run", "--release"]