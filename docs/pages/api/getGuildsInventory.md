---
title: Get Guilds's Inventory
layout: page
nav_link: Get Guilds's Inventory
nav_order: 353
nav_level: 3
lang: en
---

```
GET /v1/guilds/{guildid}/inventory
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
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

#### 200: Ok
```json
{
  "guild": {
    "type": "guild",
    "id": 12315412,
    "name": "Schrank"
  },
  "inventory": {
    "ownedbooks": [
      {
        "id": 123432,
        "quality": "Bad",
        "available": true,
        "rental": {
          "from": "1997-07-16",
          "to": "1997-07-25",
          "rentee": {
            "type": "member",
            "id": 12931,
            "name": "Eva Musterapfel"
          }
        }
      }
    ],
    "rentals": [
      {
        "id": 123432,
        "owner": {
          "type": "member",
          "id": 12931,
          "name": "Eva Musterapfel"
        },
        "quality": "Bad",
        "available": true,
        "rental": {
          "from": "1997-07-16",
          "to": "1997-07-25",
        }
      }
    ],
  }
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
