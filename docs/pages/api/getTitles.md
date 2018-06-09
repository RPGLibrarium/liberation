---
title: Get all Titles
layout: page
nav_link: Get Titles
nav_order: 321
nav_level: 3
lang: en
---
```
GET /v1/titles
```
### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=false %}

### Responses
**Content-Type:** application/json
- [200: OK](#200-ok)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401getTitles)
- [429: Too Many Requests](#429getTitles)

#### 200: OK
The response body contains a list of all registered titles in JSON format.
```json
{
  "titles": [
    {
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
    }
  ]
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/tooManyRequests.md %}
