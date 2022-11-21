FROM rust:1.65.0 as build
WORKDIR /app
COPY ./twitter-list-observer/ .
COPY ./rust-lib/ ../rust-lib/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path .
 
FROM debian:buster-slim
COPY --from=build /usr/local/cargo/bin/twitter-list-observer /usr/local/bin/app

CMD ["app"]
