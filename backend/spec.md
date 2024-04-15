# GET `/api` - returns version number for api
# GET `/cdn` - returns version number for cdn
# GET `/api/books` - return list of all book id and name's
```json
{
"books" : array{array{string}},
}
```
# POST `/api/books` - create a new book resource
```json
{
"isbn": string,
"name": string,
"authors": array{string},
"tags": array{string},
}
```

# GET `/api/{isbn}` - return details of the book
```json
{
"isbn": string,
"name": string,
"authors": array{string},
tags: array{string},
}
```

# GET `/api/{book_id}/resources` - return list of all resource id and name's
```json
{
"resources": array{array{string}}},
}
```
