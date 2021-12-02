FROM rust:latest

LABEL mantainer="Victor Torres <vtt0001@correo.ugr.es>"
LABEL vtt0001.NewPhone.url="https://github.com/vtt0001/NewPhone>"

RUN mkdir -p /app/test\
    && apt-get update && apt-get upgrade -y 

WORKDIR /app/test

CMD ["cargo", "test"]