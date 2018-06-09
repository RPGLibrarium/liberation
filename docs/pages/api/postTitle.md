---
title: Create Title
layout: page
nav_link: Create Title
nav_order: 323
nav_level: 3
lang: en
---
```
POST /v1/titles
```
### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Request Body
```json
{
  "title": {
      "name": "Wege der Helden",
      "system": 84323456,
      "language": "DE",
      "publisher": "Ulisses",
      "year": 2007,
      "coverimage": "https://example.com/wege-der-helden.jpg",
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

#### 201: Created
Headers:
```http
HTTP/1.1 201 Created
Location: /v1/titles/<new id>
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/tooManyRequests.md %}
