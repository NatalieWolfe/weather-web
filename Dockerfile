FROM rust:1

WORKDIR /opt/weather/web
COPY . .
RUN cargo install --path .

ENV PORT 8080
EXPOSE ${PORT}

CMD ["weather-web"]
