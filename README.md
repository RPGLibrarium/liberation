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
```shell
diesel migration run
```
