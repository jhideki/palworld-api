# Use a Rust base image for building
FROM rust:latest 

WORKDIR /app/palworld-api

# Copy project files
COPY . .

# Build the application with static linking
RUN cargo build --release

CMD ["target/release/palworld-api"]

# Expose port if needed
EXPOSE 8000
