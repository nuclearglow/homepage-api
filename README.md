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

* fire up postgresql

```
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=password postgres:12
```

* initialize database and run migrations

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
