---
title: Get Guilds Details
layout: page
nav_link: Get Guild Details
nav_order: 352
nav_level: 3
lang: en
---
```
GET /v1/guilds/{guildid}
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Responses
**Content-Type:** application/json
- [200: OK](#200-ok)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

#### 200: Ok
```json
{
  "guild": {
    "id": "123143",
    "name": "Librarium Kapu2",
    "address": "Schusterstraße 23, 12345 Entenhausen",
    "contact": {
      "id": "123143",
      "name": "Eva Musterapfel",
      "email": "eva.musterapfel@example.com"
    }
  },
  "editable": true,
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
