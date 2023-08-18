FROM lukemathwalker/cargo-chef:latest-rust-latest as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for the project.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build the project dependencies, not the application.
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point if the dependency tree stays the same, all lawyers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build the project
RUN cargo build --release --bin zero2prod

FROM debian:stable-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/list/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./zero2prod" ]