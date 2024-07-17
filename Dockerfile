# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update
RUN apt-get upgrade  -y
RUN apt-get install musl-tools  -y
RUN apt-get install pkg-config libssl-dev  -y
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/prompte-node
ENV ENV=dev
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc&SQLX_OFFLINE=true&DATABASE_URL=postgresql://norbert:HZ6HdG5KyP8Khtf8ZQKfMo2VUKhCtvLv@database-1.cdiyuyqkk565.ap-southeast-1.rds.amazonaws.com:5432/norbert?currentSchema=public cargo build --release --target=x86_64-unknown-linux-musl
RUN ls -l /usr/src/prompte-node/target/x86_64-unknown-linux-musl/release/
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 promote
RUN adduser -D -s /bin/sh -u 1000 -G promote promote
WORKDIR /home/promote/bin/
COPY --from=cargo-build /usr/src/prompte-node/target/x86_64-unknown-linux-musl/release/promote-node .
COPY --from=cargo-build /usr/src/prompte-node/.dev.env .
RUN chown promote:promote promote-node
ENV ENV=dev
USER promote
expose 8000

CMD ["./promote-node"]