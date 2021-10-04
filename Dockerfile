FROM alpine:latest

WORKDIR /app

COPY target/x86_64-unknown-linux-musl/release/echo-service-rust .

EXPOSE 8080

CMD [ "/app/echo-service-rust" ]

