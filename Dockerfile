FROM rust:1.28-slim as builder
WORKDIR /usr/src/nabu
VOLUME ["./target", "~/.cargo"]
COPY ./docker/cargo_config $HOME/.cargo/config
COPY ./docker/debian_stretch_mirror.list /etc/apt/sources.list

# Update Cargo index
RUN cargo search 1> /dev/null
RUN apt-get update &&\
    apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config
COPY . .
RUN cargo build --release

#####################################

FROM debian:stretch-slim as prod
WORKDIR /nabu/
COPY ./docker/debian_stretch_mirror.list /etc/apt/sources.list

RUN apt-get update &&\
    apt-get install -y --no-install-recommends \
    libssl-dev pkg-config ca-certificates &&\
    apt-get clean
COPY --from=0 /usr/src/nabu/target/release/nabu .
CMD ["./nabu"]
