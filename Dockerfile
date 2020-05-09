FROM rust:1.43 as builder
WORKDIR /usr/src/rustymon
COPY . .
RUN cargo install --path .

FROM ubuntu
RUN apt update && apt install -y ca-certificates libssl1.1
COPY --from=builder /usr/local/cargo/bin/rustymon /usr/local/bin/rustymon
CMD ["rustymon"]
