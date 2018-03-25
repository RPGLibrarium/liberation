---
title: Create Guild
layout: page
nav_link: Create Guild
nav_order: 354
nav_level: 3
lang: en
---
```
POST /v1/guilds
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

#### Request Body
```json
{
  "guild": {
    "name": "Librarium Kapu2",
    "address": "Schusterstraße 23, 12345 Entenhausen",
    "contact": 237345643245
  }
}
```

### Responses
**Content-Type:** application/json
- [201: Created](#201-created)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

### Responses
**Content-Type:** application/json
- [201: Created](#201-created)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

#### 201: Created
Headers:
```http
Location: /v1/guilds/<new id>
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/tooManyRequests.md %}
