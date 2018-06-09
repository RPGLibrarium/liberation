---
title: Update Title
layout: page
nav_link: Update Title
nav_order: 324
nav_level: 3
lang: en
---

```
PUT /v1/titles/{titleid}
```
### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

#### Request Body
```json
{
  "title":
  {
    "name": "Wege der Helden",
    "system": 3042975,
    "language": "DE",
    "publisher": "Ulisses",
    "year": 2007,
    "coverimage": "https://example.com/wege-der-helden.jpg",
  }
}
```

### Responses
**Content-Type:** application/json
- [204: No Content](#204-no-content)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

#### 204: No Content

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
