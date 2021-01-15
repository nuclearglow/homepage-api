# homepage-api

* inspired by https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
* inspired by https://github.com/ManifoldFR/rustfullstack

my website rust microservice api

## Setup

* create a `.env` file with the following contents and replace the <>:

```
SERVER_URL=<your_server_url:your_port>
MYSQL_USER=<mysql-user>
MYSQL_ROOT_PASSWORD=<mysql-password>
MYSQL_DATABASE=<mysql-database>
```

* spin up a MySQL backend via docker-compose

```
docker-compose up
```

* test connection to db locally using mysql-client

```
sudo apt-get install mysql-client
mysql -u<mysql-user> -h 0.0.0.0 -P3307 -p<mysql-password>
```
