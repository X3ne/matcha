FROM rust:1.81.0-bullseye AS build

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools libpq-dev libssl-dev pkg-config

COPY migrations ./migrations
COPY Makefile ./Makefile
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY src ./src
COPY deployments ./deployments
COPY templates ./templates
COPY crates ./crates
COPY .sqlx ./.sqlx

RUN cargo build --release

FROM rust:1.81.0-bullseye
LABEL org.opencontainers.image.source=https://github.com/x3ne/matcha

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

RUN cargo install --version 0.8.3 sqlx-cli --locked --no-default-features --features native-tls,postgres

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/Makefile ./Makefile
COPY --from=build /app/templates ./templates

COPY --from=build /app/target/debug/matcha-back .

COPY --from=build /app/deployments/scripts/entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
