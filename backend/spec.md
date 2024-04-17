Here's the updated API specification with the new code changes:

# Books and Resources API Specification

## Overview

The Books and Resources API allows you to manage books and their associated resources. It provides endpoints for retrieving, creating, and updating books, resources, and resource collaboration scores.

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

### Get Full Books (with Resources)

**Endpoint**: `GET /api/fullbooks`

**Response**:
```json
[
  {
    "isbn": 9781009071888,
    "title": "IB Physics Coursebook",
    "authors": [
      "K. A. Tsokos"
    ],
    "image_url": "https://m.media-amazon.com/images/I/31McpVemeXL.jpg",
    "resources": [
      {
        "id": "1",
        "book_isbn": 9781009071888,
        "title": "Ex. resource",
        "author": "Dhairya Shah",
        "description": "This is an example resource",
        "file_name": "test.txt",
        "page_number": "12",
        "collab_score": 0
      }
    ]
  }
]
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
    "file_name": "test.txt",
    "page_number": "12",
    "collab_score": 0
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
  "file_name": "another.pdf",
  "page_number": "12",
  "collab_score": 0
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
  "file_name": "another.pdf",
  "page_number": "12",
  "collab_score": 0
}
```

### Upload a Resource File

**Endpoint**: `POST /api/resources/{isbn}/{resource_id}`

**Request Body**: The content of the resource file.

**Response**:
```json
"2"
```

### Update a Resource Collaboration Score

**Endpoint**: `POST /api/resources/{isbn}/{resource_id}/{score}`

**Response**:
```
Status Code: 201 Created
```

## Error Handling

The API will return appropriate HTTP status codes and error messages for any invalid requests or server-side errors.

## Authentication and Authorization

This API does not currently have any authentication or authorization mechanisms. All endpoints are publicly accessible.

## Conclusion

This API allows you to manage books and their associated resources, including the ability to retrieve, create, and update books, resources, and resource collaboration scores. If you have any further questions or need more information, please don't hesitate to ask.
