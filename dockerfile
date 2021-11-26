FROM rust:latest

LABEL mantainer="Victor Torres <vtt0001@correo.ugr.es>"
LABEL vtt0001.NewPhone.url="https://github.com/vtt0001/NewPhone>"

COPY ./Cargo.toml .

RUN mkdir -p /app/test\
    && mkdir src && touch src/main.rs\
    && apt-get update && apt-get upgrade -y \
    && useradd -ms /bin/bash noroot \
    && rm Cargo.toml

WORKDIR /app/test

CMD ["cargo", "test"]