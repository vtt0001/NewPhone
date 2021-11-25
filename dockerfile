FROM alpine:3.14

LABEL mantainer="Victor Torres <vtt0001@correo.ugr.es>"
LABEL vtt0001.NewPhone.url="https://github.com/vtt0001/NewPhone>"

ENV RUSTUP_HOME=/opt/rust CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH 
ENV PKG_CONFIG_PATH=/dlib-install/usr/local/lib64/pkgconfig/

COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN apk update \
    && apk upgrade \
    && mkdir src && touch src/main.rs\
    && mkdir -p /app/test\
    && for i in openssl-dev gcc musl-dev rust cargo; do apk add "$i"; done\
    && cargo install --force cargo-make\
    && cargo install --force cargo-chef\
    && cargo chef prepare --recipe-path recipe.json\
    && cargo chef cook --release --recipe-path recipe.json\
    && rm -r src\
    && adduser -D noroot \
    && chown -R noroot /app/test

WORKDIR /app/test

USER noroot
CMD ["makers", "test"]