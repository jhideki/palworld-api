# Use a Rust base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/palworld-api

# Copy the dependency manifest
COPY Cargo.toml Cargo.lock ./
COPY . .
COPY .env . 

# Build the application
RUN cargo build --release

# Specify the command to run your application
CMD ["target/release/palworld-api"]
