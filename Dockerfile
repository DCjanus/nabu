FROM clux/muslrust:stable as builder

WORKDIR /usr/src/nabu
COPY ./docker/cargo_config $HOME/.cargo/config
COPY . .

RUN cargo build --release

#####################################

FROM alpine:latest as prod
WORKDIR /nabu/
RUN apk add --no-cache ca-certificates postgresql-client

COPY --from=0 /usr/src/nabu/target/x86_64-unknown-linux-musl/release/nabu .
COPY ./docker/run_web.sh .
RUN chmod +x run_web.sh
CMD ["./run_web.sh"]
