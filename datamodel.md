# Technologies:
- [Rust](https://www.rust-lang.org/en-US/)
- [Keycloak](https://www.keycloak.org/index.html)

# Objects
## Book
- Title
- Owner
- List\<Rental\>
- Quality

# Title:
- Name
- System
- Language
- Publisher
- Year
- Coverimage
# Rental:
- Anfangsdatum
- Enddatum
- Member

## Rpgsystem
- Name

## Members
- Username
- Name
- EMail
- Role
-
## Projects



# APIs
## Book Managment API
### Get all registered books
```
GET /v1/books
```
#### Parameters
| Name | Type  | Required | Description |
|--------------|--------|----------|----------------------------------------------------------------------------------|
|  |  |  |
| Authorization | header | true |  |
| Content-type | header | false | Used to specify the content type of the request data. Must be `application/json` |
|  |  |  |  |
|  |  |  |  |
#### Responses
**Content-Type:** application/json
- [200: OK](#200-ok-books)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [403: Forbidden](#403-forbidden)
- [429: Too Many Requests](#429-too-many-requests)

##### 200: OK {#200-ok-books}
The response body contains a list of all registered books in JSON format.
```json
{
  "lastPage": false,
  "result": "sucess",
  "books": [
    {
      "bookid": "b102934",
      "title": {
        "titleid": "t123829",
        "title": "Wege der Helden",
        "system": "DSA 4.1",
        "language": "DE",
        "publisher": "Ulisses",
        "year": "2007",
        "coverimage": "https://example.com/wege-der-helden.jpg"
      },
      "owner": "u1239288"
    }
    {
      "bookid": "b102934",
      "title": {
        "titleid": "t123829",
        "title": "Wege der Helden",
        "system": "DSA 4.1",
        "language": "DE",
        "publisher": "Ulisses",
        "year": "2007",
        "coverimage": "https://example.com/wege-der-helden.jpg"
      },
      "owner": "u1239288"
    }
    {
      "bookid": "b102934",
      "title": {
        "titleid": "t123829",
        "title": "Wege der Helden",
        "system": "DSA 4.1",
        "language": "DE",
        "publisher": "Ulisses",
        "year": "2007",
        "coverimage": "https://example.com/wege-der-helden.jpg"
      },
      "owner": "u1239288"
    }
  ]
}
```
### Get all books by system
```
GET /v1/books/rpgsystem/{systemid}
```
### Get all books by owner
```
GET /v1/books/owner/{memberid}
```
### Get book informations
```
GET /v1/books/{bookid}
```

## Member Managment API
### Get all active members
```
GET /v1/members/
```

### Get all members
```
GET /v1/members/private
```

### Get member informations
```
GET /v1/members/{memberid}
```

## Project Managment API
### Get all projects
```
GET /v1/projects
```
### Get project informations
```
GET /v1/projects/{projectid}
```

## Generic Responses
#### 400: Bad Request
#### 401: Unauthroized
#### 403: Forbidden
#### 429: Too Many Requests
