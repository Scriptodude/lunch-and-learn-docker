# Launch using
# docker run --rm -d -p '127.0.0.1:8000':8000 -e SQL_HOST=mysql_techso -e SQL_PORT=3306 --network fullstack --name backend lunch_and_learn:backend

FROM rust:1.40 as builder
MAINTAINER Jonathan Lavigne - jlavigne@techso.com


WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:latest
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
EXPOSE 8000
CMD ["backend"]