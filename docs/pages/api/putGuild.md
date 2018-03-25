---
title: Update Guild
layout: page
nav_link: Update Guild
nav_order: 355
nav_level: 3
lang: en
---

```
PUT /v1/guilds/{guildid}
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Request Body
```json
{
  "guild": {
    "name": "Librarium Kapu2",
    "address": "Schusterstraße 23, 12345 Entenhausen",
    "contact": {
      "id": "123143",
    }
  }
}
```

### Responses
**Content-Type:** application/json
- [204: No Content](#204-no-content)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

#### 204: No Content


{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
