FROM rust:1.70.0-bullseye AS build
ADD . /app
WORKDIR /app
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apt-get install -y git yarn \
    && cd frontend && yarn install && yarn run build && cd ..
    && cargo build -p typst-book --release

FROM debian:11
WORKDIR /root/
COPY --from=build  /app/target/release/typst-book /bin
