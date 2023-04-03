FROM alpine:latest as build

RUN apk upgrade && \
    apk add curl gcc musl-dev pkgconfig openssl openssl-dev build-base make cmake zlib bash && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain stable --component rust-src -y


WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN source $HOME/.cargo/env && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/wawa /wawa 


FROM alpine

COPY --from=build /wawa /wawa

CMD ["./wawa"]
