---
title: Get Books
layout: page
nav_link: Get Books
nav_order: 331
nav_level: 3
lang: en
---

```
GET /v1/books/
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
  "books": [
    {
      "id": 123432,
      "title": {
        "id": 123829,
        "name": "Wege der Helden",
        "system": {
          "id": 3042975,
          "name": "DSA 4.1"
        },
        "language": "DE",
        "publisher": "Ulisses",
        "year": 2007,
        "coverimage": "https://example.com/wege-der-helden.jpg",
        "stock": 5,
        "available": 5,
      },
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
        "rentee": {
          "type": "member",
          "id": 12931,
          "name": "Eva Musterapfel"
        }
      }
    }
  ]
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/tooManyRequests.md %}
