#### 429: Too Many Requests
Right now, the API does not throttle your requests. You should never get this type of error.

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json
Date: Fri, 19 Jan 2018 10:31:43 GMT
Retry-After: 38
Server: APIP
X-Request-Id: iEUtsLiFgj3R4xsbirAyZlMyaxRTo8Xo
Content-Length: 54
Connection: keep-alive
```
