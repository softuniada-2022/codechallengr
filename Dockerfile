FROM rust:1.58-slim

WORKDIR /app

RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev

# a hack to cache dependencies
COPY Cargo.toml .
RUN mkdir src
RUN touch src/lib.rs
RUN cargo build --release
RUN rm src/lib.rs

COPY . .
RUN cargo build --release

CMD ["target/release/main"]
