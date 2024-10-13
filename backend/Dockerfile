FROM ubuntu:24.04


COPY public /home/public
COPY Rocket.toml /home/Rocket.toml
COPY target/x86_64-unknown-linux-musl/release/DHBackend /home/main

WORKDIR /home

RUN chmod 755 /home/main

ENV PATH="${PATH}:/home"

EXPOSE 80

CMD ["main"]