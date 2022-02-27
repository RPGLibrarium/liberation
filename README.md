# Liberation Backend

## Development setup
Dependencies (Ubuntu):
- gcc
- libssl-dev
- libmysqlclient-dev

1. Install mariadb container
```shell
podman run -dt -p 127.0.0.1:3306:3306  --name liberation-dev-db  --env MARIADB_USER=liberation --env MARIADB_PASSWORD=liberation --env MARIADB_ROOT_PASSWORD=root --env MARIADB_DATABASE=liberation docker.io/mariadb:latest
```
Or start container
```shell
podman start liberation-dev-db
```

2. Apply the latest migration
One time and update when necessary
```shell
cargo install diesel_cli
```

On first run and after each database change
```shell
diesel migration run --database-url mysql://liberation:liberation@127.0.0.1:3306/liberation
```

3. Run Liberation
```shell
cargo run -p liberation -- -d mysql://liberation:liberation@127.0.0.1:3306/liberation
```

## A short introduction to access control
### Definitions
A `client` is an application accessing resources (e.g. web frontend, Android app).
A `subject` is the authenticated entity ob behalf a client is acting (e.g. the user).
A `scope` is a subset of privileges available to a user (e.g. see account information, see collection, lend books from inventory).
The client can request certain scopes and (if they are available to the user) the user can delegate them the client.

In Liberation the access control is partially handled through scopes and partially done on application level. Scopes
control which subset privileges the user provides to the client. 

### Scopes
Liberation requires scopes for certain actions. The client can request those scopes for a user.
The web frontend asks of some scopes by default, because they are needed for the basic functionality:
- `account:read`
- `collection:read`
- `collection:modify`
- `inventory:read`
- `inventory:modify`
- `librarian:read`
- `librarian:modify`

Some more important scopes need to be requested explicitly before performing the action:
- `account:register`
- `account:delete`
- `account:modify`
- `aristocrat:read`
- `aristocrat:modify`

Not all scopes are available to all subjects. For example librarian scopes are only available to librarians and 
aristocrat scopes can only be attained by aristocrats.

### Groups and roles
The following groups are managed in keycloak
- `librarium`
  - `members`
    - `board`
    - `developers`

All members get the `liberation-user` role. All `board` members are assigned the `liberation-aristocrat` and 
the `liberation-librarian` role.

All developers get the `liberation-frontend/developer` role so that they see experimental frontend features.
