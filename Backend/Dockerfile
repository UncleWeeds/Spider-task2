

FROM rust:latest

RUN apt-get update && apt-get install -y postgresql-client

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev


RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./wait-for-postgres.sh"]
