# APIs
## Book Managment API
### Get all registered titles
```
GET /v1/titles
```
#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | false | Oauth token |
#### Responses
**Content-Type:** application/json
- [200: OK](#200-ok-titles)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [429: Too Many Requests](#429-too-many-requests)

##### 200: OK {#200-ok-titles}
The response body contains a list of all registered titles in JSON format.
```json
{
  "titles": [
    {
      "id": "123829",
      "name": "Wege der Helden",
      "system": {
        "id": "3042975",
        "name": "DSA 4.1"
      },
      "language": "DE",
      "publisher": "Ulisses",
      "year": "2007",
      "coverimage": "https://example.com/wege-der-helden.jpg",
      "stock": 5,
      "avaliable": 5,
    }
  ]
}
```

### Get title informations
```
GET /v1/titles/{titleid}
```
#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | false | Oauth token |

#### Responses
**Content-Type:** application/json
- [200: OK](#200-ok-titles)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

##### 200: OK {#200-ok-titleid}
The response body contains a list of all registered titles in JSON format.
```json
{
  "title":
  {
    "id": "123829",
    "name": "Wege der Helden",
    "system": {
      "id": "3042975",
      "name": "DSA 4.1"
    },
    "language": "DE",
    "publisher": "Ulisses",
    "year": "2007",
    "coverimage": "https://example.com/wege-der-helden.jpg",
    "stock": 5,
    "avaliable": 5,
    "books": [
      {
        "id": "123432",
        "owner": {
          "type": "member",
          "id": "12931",
          "name": "Eva Musterapfel"
        },
        "quality": "Bad",
        "avaliable": true,
        "rental": {
          "from": "1997-07-16",
          "to": "1997-07-25",
          "rentee": {
            "type": "member",
            "id": "12931",
            "name": "Eva Musterapfel"
          }
        }
      }
    ]
  }
}
```
### Get all rpg systems
```
GET /v1/rpgsystems
```

#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | false | Oauth token |

#### Responses
**Content-Type:** application/json
- [200: OK](#200-ok-titles)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [429: Too Many Requests](#429-too-many-requests)

##### 200: Ok {#200-ok-rpgsystem}
```json
{
  "rpgsystems": [
    {
      "id": "123829",
      "name": "Wege der Helden",
    }
  ]
}
```

### Get all titles by system
```
GET /v1/rpgsystem/{systemid}
```
#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | false | Oauth token |


```json
{
  "rpgsystem": {
    "id": "123829",
    "name": "Wege der Helden",
    "titles": [
      {
        "id": "123829",
        "name": "Wege der Helden",
        "system": {
          "id": "3042975",
          "name": "DSA 4.1"
        },
        "language": "DE",
        "publisher": "Ulisses",
        "year": "2007",
        "coverimage": "https://example.com/wege-der-helden.jpg",
        "stock": 5,
        "avaliable": 5,
      }
    ]
  }
}
```

#### Responses
**Content-Type:** application/json
- [200: OK](#200-ok-titles)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [404: Not Found](#404-not-found)
- [429: Too Many Requests](#429-too-many-requests)

### Get all active members
```
GET /v1/members/
```

#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | true | Oauth token |

```json
{
  "members":[
    {
      "id": "123143",
      "name": "Eva Musterapfel",
      "email": "eva.musterapfel@example.com",
      "roles": [
        {
          "identifier": "admin"
        }
        {
          "identifier": "member"
        }
      ]
    }
  ]
}
```

### Get all books of owner
```
GET /v1/members/{memberid}
```

#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
| Authorization | header | true | Oauth token |

```json
{
  "member": {
    "id": "123143",
    "name": "Eva Musterapfel",
    "email": "eva.musterapfel@example.com",
    "roles": [
      {
        "identifier": "admin"
      }
      {
        "identifier": "member"
      }
    ],
    "inventory": [

    ]
  }
}
```
### Get member informations
```
GET /v1/members/{memberid}
```

### Get all guilds

## Generic Responses
#### 400: Bad Request
#### 401: Unauthroized
#### 403: Forbidden
#### 404: Not Found
#### 429: Too Many Requests
