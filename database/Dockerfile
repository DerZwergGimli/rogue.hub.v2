FROM rust:latest

# Install SQLx CLI
RUN cargo install sqlx-cli

# Set working directory
WORKDIR /app

# Copy migration files

COPY database/migrations ./migrations
COPY database/entrypoint.sh .

# Make the entrypoint script executable
RUN chmod +x ./entrypoint.sh



# Set the DATABASE_URL environment variable when running the container
ENV DATABASE_URL=postgres://username:password@db:5432/dbname

# Command to run migrations
ENTRYPOINT ["./entrypoint.sh"]
