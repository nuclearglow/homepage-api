# homepage-api

inspired by
* https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
* https://github.com/ManifoldFR/rustfullstack
* diesel.rs

my website rust microservice api

## Setup

* fire up postgresql

```
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=password postgres:12
```

* create a `.env` file with the following contents or replace with your settings:

```
API_SERVER_URL=0.0.0.0:9090
DATABASE_URL=postgres://postgres:password@localhost:5432/api
WEBAUTHN_RELYING_PARTY_NAME=localhost
WEBAUTHN_RELYING_PARTY_ORIGIN=https://localhost:8888
WEBAUTHN_RELYING_PARTY_ID=localhost
```

* initialize database and run migrations

```
diesel setup
```

* run, test some stuff using Postman: `docs/api.svenvowe.de.postman_collection.json`
* or use the experimental frontend: https://github.com/nuclearglow/retrolist
