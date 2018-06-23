[![Build Status](https://travis-ci.org/RPGLibrarium/Liberation.svg?branch=master)](https://travis-ci.org/RPGLibrarium/Liberation)
[![dependency status](https://deps.rs/repo/github/RPGLibrarium/Liberation/status.svg)](https://deps.rs/repo/github/RPGLibrarium/Liberation)
# Liberation

Liberation is a rental system programmed exactly to the needs of the [RPG Librarium Aachen e.V.](https://rpg-librarium.de)
It manages rentals of books, owned by the members or the association, in a peer-to-peer fashion.

### Technologies:
- [Rust](https://www.rust-lang.org/en-US/)
- [Keycloak](https://www.keycloak.org/index.html)

### Documentation:
Latest docs hosted on [Github.io](https://rpglibrarium.github.io/Liberation/)

### Test:
The tests need a MySQL/MariaDB Database runing. You can use the docker containers provided:
```
docker-compose up -d
```
Set connection settings as environmental variables:
```
# export SQL_SERVER=127.0.0.1:33062
export SQL_SERVER=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' liberation_test-db_1)
export SQL_USER=root
export SQL_PASSWORD=thereIsNoPassword!
```
Run Tests with cargo:
```
cargo test
```

### Build:
Use cargo
```
cd Liberation
cargo build
```
