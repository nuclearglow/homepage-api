# homepage-api

inspired by
* https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
* https://github.com/ManifoldFR/rustfullstack
* diesel.rs

my website rust microservice api

## Setup

* create a `.env` file with the following contents and replace the <>:

```
SERVER_URL=<your_server_url:your_port>
DATABASE_URL=mysql://api:api@localhost:3307/api
```

* install diesel

Use this [guide](https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md#mysql):

```bash
wget https://dev.mysql.com/get/mysql-apt-config_0.8.15-1_all.deb
sudo dpkg -i mysql-apt-config_0.8.15-1_all.deb
sudo apt-get update
sudo apt-get install libmysqlclient-dev
cargo install diesel_cli --no-default-features --features mysql
```

* spin up a MySQL backend via docker-compose

```
docker-compose up
```

* test connection to db locally using mysql-client and set up the database

```bash
sudo apt-get install mysql-client
mysql -u<mysql-user> -h 0.0.0.0 -P3307 -p<mysql-password>
```

```sql
CREATE DATABASE <mysql-database> CHARACTER SET utf8 COLLATE 'utf8_general_ci';
CREATE USER '<mysql-user>'@'localhost' IDENTIFIED BY '<mysql-password>';
GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, INDEX, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON <mysql-database>.* TO '<mysql-user>'@'localhost';
GRANT FILE ON *.* TO '<mysql-user>'@'localhost';
```

* For remote MySQL Access (like to the docker image), also run:

```sql
CREATE USER '<mysql-user>'@'%' IDENTIFIED BY '<mysql-password>';
GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, INDEX, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON <mysql-database>.* TO '<mysql-user>'@'%';
GRANT FILE ON *.* TO '<mysql-user>'@'%';
```

* initialize database

```
diesel setup
```



# Notes

* first migration was:

```
diesel migration generate create_lists
```

* Database migrations for prod:

See https://diesel.rs/guides/getting-started/:

When preparing your app for use in production, you may want to run your migrations during the application's initialization phase. You may also want to include the migration scripts as a part of your code, to avoid having to copy them to your deployment location/image etc.

The diesel_migrations crate provides the embed_migrations! macro, allowing you to embed migration scripts in the final binary. Once your code uses it, you can simply include embedded_migrations::run(&db_conn) at the start of your main function to run migrations every time the application starts.

