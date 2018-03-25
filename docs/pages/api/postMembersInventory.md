---
title: Add to Member's Inventory
layout: page
nav_link: Add to Member's Inventory
nav_order: 344
nav_level: 3
lang: en
---

```
POST /v1/members/{memberid}/inventory
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Request Body
```json
{
  "book": {
    "title": "567548",
    "quality": "good"
  }
}
```

### Responses
**Content-Type:** application/json
- [201: Created](#201-created)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

#### 201: Created
Headers:
```http
Location: /v1/books/<new id>
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
