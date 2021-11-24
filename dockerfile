FROM alpine:3.14

LABEL mantainer="Victor Torres <vtt0001@correo.ugr.es>"

ENV RUSTUP_HOME=/opt/rust CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH 
ENV PKG_CONFIG_PATH=/dlib-install/usr/local/lib64/pkgconfig/

WORKDIR /app/test
USER root

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./Makefile.toml .

RUN apk update \
    && apk upgrade \
    && mkdir src && touch src/main.rs\
    && for i in openssl-dev gcc musl-dev rust cargo; do apk add "$i"; done\
    && cargo install --force cargo-make\
    && cargo install --force cargo-chef\
    && cargo chef prepare --recipe-path recipe.json\
    && cargo chef cook --release --recipe-path recipe.json\
    && rm src/main.rs\
    && adduser -D noroot \
    && chown -R noroot /app/test
    

COPY src ./src

USER noroot
CMD ["makers", "test"]


