# Listing people api actix

Example of application of listing people

## Run project

* set postgres
* create .env file

```
DATABASE_URL=postgres://[YOUR USER]:[YOUR PASSWORD]@localhost/[YOUR DATABASE]
URL=localhost:8000
GRAPHQL_URL=localhost:8088
```

* install diesel client 

`cargo install diesel_cli`

* execute the migrations

`diesel setup`
`diesel migration generate create_person`

* write in the file up.sql generated inside of the folde migrations/diesel_initial_setup

```sql
CREATE TABLE people (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name VARCHAR(250) NOT NULL,
  super_power BOOLEAN NOT NULL DEFAULT FALSE,
  rich BOOLEAN NOT NULL DEFAULT FALSE,
  genius BOOLEAN NOT NULL DEFAULT FALSE,
  UNIQUE(name)
);
```

* execute `diesel migration run`
* run `cargo run`

## Test the project

`cargo test --test people_test -- --nocapture --test-threads=1`
