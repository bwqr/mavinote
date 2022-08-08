FROM rust:1-slim-bullseye as builder

WORKDIR /usr/src/mavinote

COPY . .

RUN apt-get update && apt-get install -y libpq-dev libsqlite3-dev && cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 libsqlite3-0

WORKDIR /usr/local/apps/mavinote

COPY --from=builder /usr/src/mavinote/target/release/backend ./mavinote

EXPOSE 8050

CMD ["./mavinote"]
