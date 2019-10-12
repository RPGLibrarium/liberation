FROM rust

COPY . /build
WORKDIR /build

RUN cargo install --path . \
 && cargo clean \
 && mkdir -p /app \
 && cp -r config res web /app/ \
 && rm -rf /build

WORKDIR /app

CMD ["liberation"]

EXPOSE 8080
