---
title: Create Book
layout: page
nav_link: Create Book
nav_order: 334
nav_level: 3
lang: en
---

```
POST /v1/books/
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Reuquest Body
```json
{
  "book": {
    "title": "567548",
    "owner": {
      "type": "member",
      "id": "123456754"
    },
    "quality": "Bad"
  }
}
```

### Responses
**Content-Type:** application/json
- [201: Created](#200-created)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

Allows to omit `owner` and assumes the current logged in member by default.

#### 201: Created
Headers:
```http
Location: /v1/books/<new id>
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/tooManyRequests.md %}
