FROM dejan1998/bionic-rust:latest
LABEL maintainer="Giovanni Dejan <giovanni.dejan@binus.ac.id>"

WORKDIR /app
COPY . .

RUN cargo build --workspace --verbose

CMD ["bash", "validator.sh"]
