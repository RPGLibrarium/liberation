
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
