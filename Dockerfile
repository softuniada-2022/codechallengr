# https://stackoverflow.com/a/64528456
FROM rust:1.58-slim AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM rust:1.58-slim AS diesel
WORKDIR /app
RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev
RUN cargo install diesel_cli --no-default-features --features mysql
RUN which diesel
RUN cp /usr/local/cargo/bin/diesel .

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin main

FROM node:17-alpine as vue_builder
WORKDIR /app/vue
COPY vue .
RUN yarn install
RUN yarn build

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/main /usr/local/bin
COPY --from=vue_builder /app/vue/dist dist
COPY --from=diesel /app/diesel /usr/local/bin
COPY entry.sh .
RUN chmod +x entry.sh
ENTRYPOINT ["/app/entry.sh"]
