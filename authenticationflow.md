
## [2.4.2. Validating Access Tokens](https://www.keycloak.org/docs/4.0/securing_apps/index.html#validating-access-tokens)
If you need to manually validate access tokens issued by Keycloak you can invoke the Introspection Endpoint. The downside to this approach is that you have to make a network invocation to the Keycloak server. This can be slow and possibily overload the server if you have too many validation requests going on at the same time. Keycloak issued access tokens are JSON Web Tokens (JWT) digitally signed and encoded using JSON Web Signature (JWS). Because they are encoded in this way, **this allows you to locally validate access tokens using the public key of the issuing realm.** You can either hard code the realm’s public key in your validation code, or lookup and cache the public key using the certificate endpoint with the Key ID (KID) embedded within the JWS. Depending what language you code in, there are a multitude of third party libraries out there that can help you with JWS validation.

# Authenticaion flow
Username: dummy
Passwor: dummy

## Public vs Private Client:
Public Client: Webapp
Private Client: Github server, Cronjob

## [Setup Client in Keycloak](https://www.keycloak.org/docs/4.0/securing_apps/index.html#_javascript_adapter)
- Client: Liberation Webapp
  - AccessType: Public
  - Redirect callbacks must be specific. i.e: https://liberation.rpg-librarium.de/


## Flows
- Passwort Credential, Direct Access Grant -> NOOO!
- Implicit -> No, since Access Token(with long validity is sent back to the client through the redirect/callback URL)
- Authorization Code -> YES

1. Redirect to authorization URL (Keycloak)
2. Get Authorization Token for Client (Webapp)
3. Use Authorization Token to get Access Token by (Keycloak)

## UserDB Access with Service Account access
- Service Aacount Enabled
- Generate Client Secret
- `echo -n "liberation-core:b419cbee-9187-470e-bded-b8649d5cc18e"| base64`
- `curl -H "Authorization: Basic bGliZXJhdGlvbi1jb3JlOmI0MTljYmVlLTkxODctNDcwZS1iZGVkLWI4NjQ5ZDVjYzE4ZQ==" -H "Content-Type: application/x-www-form-urlencoded" -d "grant_type=client_credentials" -X POST http://localhost:8081/auth/realms/liberation/protocol/openid-connect/token`
- `curl -H "Authorization: Bearer <Token>" http://localhost:8081/auth/admin/realms/liberation/users?max=20&search=p -v`
Response:
```json
[
  {
    "id": "5b53c5bb-da9d-40b1-9309-bf47deaf8f7d",
    "createdTimestamp": 1532790611876,
    "username": "dummy",
    "enabled": true,
    "totp": false,
    "emailVerified": false,
    "disableableCredentialTypes": [
      "password"
    ],
    "requiredActions": [],
    "notBefore": 0,
    "access": {
      "manageGroupMembership": false,
      "view": true,
      "mapRoles": false,
      "impersonate": false,
      "manage": false
    }
  },
  {
    "id": "03c2ab7f-bbc9-4cf1-a477-97d919961995",
    "createdTimestamp": 1529751875413,
    "username": "paradyx",
    "enabled": true,
    "totp": false,
    "emailVerified": true,
    "firstName": "Yoann",
    "lastName": "Kehler",
    "email": "liberation@acc.yoann.de",
    "disableableCredentialTypes": [
      "password"
    ],
    "requiredActions": [],
    "notBefore": 0,
    "access": {
      "manageGroupMembership": false,
      "view": true,
      "mapRoles": false,
      "impersonate": false,
      "manage": false
    }
  },
  {
    "id": "798f70de-24fa-4ef4-bbd5-1bfe3b736bd5",
    "createdTimestamp": 1529751698619,
    "username": "richardz",
    "enabled": true,
    "totp": false,
    "emailVerified": true,
    "firstName": "Richard",
    "lastName": "Zameitat",
    "email": "r@richardz.de",
    "disableableCredentialTypes": [
      "password"
    ],
    "requiredActions": [
      "CONFIGURE_TOTP",
      "VERIFY_EMAIL",
      "UPDATE_PASSWORD",
      "UPDATE_PROFILE"
    ],
    "notBefore": 0,
    "access": {
      "manageGroupMembership": false,
      "view": true,
      "mapRoles": false,
      "impersonate": false,
      "manage": false
    }
  }
]
```
- Get all User:
```
http://localhost:8081/auth/admin/realms/liberation/users
```
- Search User:
```
http://localhost:8081/auth/admin/realms/liberation/users?max=20&search=p
```
- GetUser:
```
http://localhost:8081/auth/admin/realms/liberation/users/03c2ab7f-bbc9-4cf1-a477-97d919961995
```
