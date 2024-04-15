# Books and Resources API Specification

## Overview

The Books and Resources API allows you to manage books and their associated resources. It provides endpoints for retrieving, creating, and updating books and resources.

## Endpoints

### Get All Books

**Endpoint**: `GET /api/books`

**Response**:
```json
{
  "books": [
    {
      "isbn": 9781009071888,
      "title": "IB Physics Coursebook",
      "authors": [
        "K. A. Tsokos"
      ],
      "image_url": "https://m.media-amazon.com/images/I/31McpVemeXL.jpg"
    },
    {
      "isbn": 9780198390022,
      "title": "IB Chemistry Coursebook",
      "authors": [
        "Sergey Bylikin",
        "Christopher Talbot"
      ],
      "image_url": "https://m.media-amazon.com/images/I/51j9V7HEYAL.jpg"
    }
  ]
}
```

### Get a Single Book

**Endpoint**: `GET /api/books/{isbn}`

**Response**:
```json
{
  "isbn": 9781009071888,
  "title": "IB Physics Coursebook",
  "authors": [
    "K. A. Tsokos"
  ],
  "image_url": "https://m.media-amazon.com/images/I/31McpVemeXL.jpg"
}
```

### Get Resources for a Book

**Endpoint**: `GET /api/resources/{isbn}`

**Response**:
```json
[
  {
    "id": "1",
    "book_isbn": 9781009071888,
    "title": "Ex. resource",
    "author": "Dhairya Shah",
    "description": "This is an example resource",
    "file_name": "test.txt"
  },
  {
    "id": "2",
    "book_isbn": 9781009071888,
    "title": "Another Resource",
    "author": "Jane Doe",
    "description": "This is another resource",
    "file_name": "another.pdf"
  }
]
```

### Get a Resource File

**Endpoint**: `GET /api/resources/{isbn}/{resource_id}`

**Response**: The content of the resource file.

### Create a Book

**Endpoint**: `POST /api/books`

**Request Body**:
```json
{
  "isbn": 9780198390022,
  "title": "IB Chemistry Coursebook",
  "authors": [
    "Sergey Bylikin",
    "Christopher Talbot"
  ],
  "image_url": "https://m.media-amazon.com/images/I/51j9V7HEYAL.jpg"
}
```

**Response**:
```json
{
  "isbn": 9780198390022,
  "title": "IB Chemistry Coursebook",
  "authors": [
    "Sergey Bylikin",
    "Christopher Talbot"
  ],
  "image_url": "https://m.media-amazon.com/images/I/51j9V7HEYAL.jpg"
}
```

### Create a Resource

**Endpoint**: `POST /api/resources/{isbn}`

**Request Body**:
```json
{
  "id": "2",
  "book_isbn": 9781009071888,
  "title": "Another Resource",
  "author": "Jane Doe",
  "description": "This is another resource",
  "file_name": "another.pdf"
}
```

**Response**:
```json
{
  "id": "2",
  "book_isbn": 9781009071888,
  "title": "Another Resource",
  "author": "Jane Doe",
  "description": "This is another resource",
  "file_name": "another.pdf"
}
```

### Upload a Resource File

**Endpoint**: `POST /api/resources/{isbn}/{resource_id}`

**Request Body**: The content of the resource file.

**Response**:
```json
"2"
```

## Error Handling

The API will return appropriate HTTP status codes and error messages for any invalid requests or server-side errors.

## Authentication and Authorization

This API does not currently have any authentication or authorization mechanisms. All endpoints are publicly accessible.

## Conclusion

This API allows you to manage books and their associated resources. You can retrieve, create, and update books and resources using the provided endpoints. If you have any further questions or need more information, please don't hesitate to ask.
