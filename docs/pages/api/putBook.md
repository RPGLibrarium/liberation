---
title: Update Book
layout: page
nav_link: Update Book
nav_order: 334
nav_level: 3
lang: en
---

```
PUT /v1/books/{bookid}
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Request Body
```json
{
  "book": {
    "title": 134234,
    "owner": {
      "type": "member",
      "id": 12931,
    },
    "quality": "Bad",
  }
}
```

### Responses
**Content-Type:** application/json
- [204: No Content](#200-no-content)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

#### 204: no-content


{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
