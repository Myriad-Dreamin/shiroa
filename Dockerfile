
ARG NODE_VERSION=18
ARG RUST_VERSION=1.71.0

FROM node:${NODE_VERSION}-alpine AS build-yarn
RUN apk add --no-cache cpio findutils git
ADD . /app
WORKDIR /app
RUN cd frontend && yarn install && yarn run build

FROM rust:${RUST_VERSION}-bullseye AS build
ADD . /app
WORKDIR /app
COPY --from=build-yarn /app/frontend /app/frontend
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apt-get install -y git \
    && cargo build -p typst-book --release

FROM debian:11
WORKDIR /root/
COPY --from=build  /app/target/release/typst-book /bin
