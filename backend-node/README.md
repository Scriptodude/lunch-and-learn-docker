# backend-node

[![LoopBack](https://github.com/strongloop/loopback-next/raw/master/docs/site/imgs/branding/Powered-by-LoopBack-Badge-(blue)-@2x.png)](http://loopback.io/)

## Run using docker

1. build the docker `docker build . -t lunch_and_learn:node`
1. run the docker `docker run --rm -d -p '127.0.0.1:8000':8000 -e SQL_HOST=mysql_techso -e SQL_PORT=3306 --network fullstack --name backend2 lunch_and_learn:node`

## Run using yarn

`SQL_HOST=<your-host> SQL_PORT=<your-port> yarn start`

for instance

`SQL_HOST=localhost SQL_PORT=1337 yarn start`
