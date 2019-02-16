[![Build Status](https://travis-ci.org/RPGLibrarium/Liberation.svg?branch=master)](https://travis-ci.org/RPGLibrarium/Liberation)
[![dependency status](https://deps.rs/repo/github/RPGLibrarium/Liberation/status.svg)](https://deps.rs/repo/github/RPGLibrarium/Liberation)
# Liberation

Liberation is a rental system programmed exactly to the needs of the [RPG Librarium Aachen e.V.](https://rpg-librarium.de)
It manages rentals of books, owned by the members or the association, in a peer-to-peer fashion.

### Technologies:
- [Rust](https://www.rust-lang.org/en-US/)
- [Keycloak](https://www.keycloak.org/index.html)
- [MariaDB](https://mariadb.com/)
### Documentation:
Latest docs hosted on [Github.io](https://rpglibrarium.github.io/Liberation/)

## Setup
####Install MariaDB
####Install Keycloak
1. Start docker containers `keycloak` and `keycloak-db`
```
docker-compose pull keycloak keycloak-db
docker-compose up keycloak keycloak-db
```
2. Login on the admin console: `http://localhost:8081/auth` with `admin:admin`
3. Import realm from `liberation-realm-export.json`. Make sure you use the correct Keycloak version.
4. Reset and retrieve client-secret (Realms->Liberation->Clients->liberation-core->Credentials) for later use.
5. Allow `realm-management`/`view-users` for the Service Account Roles of `liberation-core`. For some reason this is not exported.

####Install Liberation-core
1. Build
```
cd Liberation
cargo build
```

2. Install liberation core as a service
3. Configure
<!-- Liberation will look for configurations in the environment and at `/etc/liberation/master.conf`.
Set the database connection properties and keycloak client secret like in the [example config](res/config.yml)
Make sure the config is only readable by the liberation-core service. -->
#### Install Frontend to Webserver
1. Build
2. Deploy
3. Configure
Get the `keycloak.json` from your keycloak instance installed in the previous steps. And make it accessible to the liberation-frontend

### Test:
The tests need a MySQL/MariaDB Database runing. You can use the docker containers provided:
```
docker-compose up -d
```
Set connection settings as environmental variables or in `config/test.toml`:
```
export LIBERATION_DATABASE_HOSTNAME=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' liberation_test-db_1)
export LIBERATION_DATABASE_USERNAME=root
export LIBERATION_DATABASE_PASSWORD=thereIsNoPassword!
```
Run Tests with cargo:
```
cargo test
```
