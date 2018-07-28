---
title: Update RpgSystem
layout: page
nav_link: Update RpgSystem
nav_order: 315
nav_level: 3
lang: en
---

```
PUT /v1/rpgsystems/{systemid}
```
### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=true %}

### Request Body
```json
{
  "rpgsystem": {
    "name": "Wege der Helden",
  }
}
```

### Responses
**Content-Type:** application/json
- [204: No Content](#204-no-content)
- [400: Bad Request](#400getTitlesByRpgSystem)
- [401: Unauthroized](#401getTitlesByRpgSystem)
- [403: Forbidden](#403getTitlesByRpgSystem)
- [404: Not Found](#404getTitlesByRpgSystem)
- [429: Too Many Requests](#429getTitlesByRpgSystem)

#### 204: No Content

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
