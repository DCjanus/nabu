FROM clux/muslrust:stable as builder

WORKDIR /nabu
COPY . .

ARG use_mirror
RUN if [ $use_mirror ]; then \
        mkdir -p $HOME/.cargo; \
        mv -f ./docker/cargo_config  $HOME/.cargo/config; \
    fi
RUN cargo build --release

#####################################

FROM alpine:latest as prod
EXPOSE 80
WORKDIR /nabu/
ENV LOCAL_ADDR="0.0.0.0:80"

RUN apk add --no-cache ca-certificates postgresql-client

COPY --from=0 /nabu/target/x86_64-unknown-linux-musl/release/nabu .
COPY ./docker/run_web.sh .
RUN chmod +x run_web.sh
CMD ["./run_web.sh"]
