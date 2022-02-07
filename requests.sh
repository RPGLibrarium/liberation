# Anonymous
curl -X GET localhost:8080/api/v1/rpgsystems
curl -X GET localhost:8080/api/v1/titles

# Members
## Account management
curl -H "Authorization: Bearer $TOKEN" -X POST -H 'Content-Type: application/json' localhost:8080/api/v1/me -d '{"username": "maxmusterman"}'
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/me 
## This requires manual deletion from the database, as of now, or reactivation by an aristocrat.
curl -H "Authorization: Bearer $TOKEN" -X DELETE localhost:8080/api/v1/me 

## See other stuff
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/guilds
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/guilds/0
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/users
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/users/1

## Book Management

# Librarian
## Book Management


# Aristocrats
## Account management
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/accounts
curl -H "Authorization: Bearer $TOKEN" -X GET localhost:8080/api/v1/accounts/1
curl -H "Authorization: Bearer $TOKEN" -X PUT -H 'Content-Type: application/json' localhost:8080/api/v1/accounts/1 -d '{"active":true,"external_id":"xxxxxxxx","username":"maxmustermann","full_name":"Max Mustermann","given_name":"Max","family_name":"Mustermann","email":"max.mustermann@example.com"}'
curl -H "Authorization: Bearer $TOKEN" -X DELETE localhost:8080/api/v1/accounts/1
## This requires manual deletion from the database, as of now, or reactivation by an aristocrat.
curl -H "Authorization: Bearer $TOKEN" -X DELETE localhost:8080/api/v1/accounts/1

## Guild Management
curl -H "Authorization: Bearer $TOKEN" -X POST -H 'Content-Type: application/json' localhost:8080/api/v1/guilds -d '{"external_id": "librarium", "name": "RPG Librarium Aachen e.V.", "address": "Postfach 101632, 52016 Aachen", "contact_by_account_id": 1}'
curl -H "Authorization: Bearer $TOKEN" -X PUT -H 'Content-Type: application/json' localhost:8080/api/v1/guilds/1 -d '{"external_id": "librarium", "name": "RPG Librarium Aachen e.V.", "address": "Postfach 101632, 52016 Aachen, Germany", "contact_by_account_id": 1}'

## Book Management
