FROM rust:1.76-slim-buster AS build

WORKDIR /app

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      lld libssl-dev ssh git pkg-config

ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

COPY ./crates ./crates
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release --all-targets


FROM debian:buster-slim as ndc-sendgrid

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      libssl-dev ca-certificates

WORKDIR /app
COPY --from=build /app/target/release/ndc-sendgrid ./ndc-sendgrid

RUN mkdir -p /etc/connector
ENV HASURA_CONFIGURATION_DIRECTORY=/etc/connector

ENTRYPOINT ["./ndc-sendgrid"]
CMD ["serve"]
