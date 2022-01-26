# Liberation Backend

## Development setup
1. Install mariadb container
```shell
podman run -dt -p 127.0.0.1:3306:3306  --name liberation-dev-db  --env MARIADB_USER=liberation --env MARIADB_PASSWORD=liberation --env MARIADB_ROOT_PASSWORD=root --env MARIADB_DATABASE=liberation docker.io/mariadb:lates
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
