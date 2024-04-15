use axum::{
    extract::{Path, Json},
    routing::get,
    Router, http::{StatusCode, Response}, body::{to_bytes, Body},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path as FilePath};
use tracing::info;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Book {
    isbn: u64,
    title: String,
    authors: Vec<String>,
    image_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Resource {
    id: String,
    book_isbn: u64,
    title: String,
    author: String,
    description: String,
    file_name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    if !FilePath::new("books.json").exists() {
        info!("books.json not found, creating an example book");
        let example_book = Book {
            isbn: 9781009071888,
            title: "IB Physics Coursebook".to_string(),
            authors: vec!["K. A. Tsokos".to_string()],
            image_url: Some("https://m.media-amazon.com/images/I/31McpVemeXL.jpg".to_string()),
        };

        let example_resource = Resource {
            id: "1".to_string(),
            book_isbn: example_book.isbn,
            title: "Ex. resource".to_string(),
            author: "Dhairya Shah".to_string(),
            description: "This is an example resource".to_string(),
            file_name: "test.txt".to_string(),
        };
        let books: Vec<Book> = vec![example_book.clone()];
        let books_json = serde_json::to_string(&books).unwrap();
        fs::write("books.json", books_json).unwrap();
        save_resource(example_resource.clone());
        info!("Example book and resource saved");
    } else {
        info!("books.json found, loading existing data");
    }

    let app = Router::new()
        .route("/api/books", get(get_books).post(post_book))
        .route("/api/books/:isbn", get(get_book))
        .route("/api/resources/:isbn", get(get_resources).post(post_resources))
        .route("/api/resources/:isbn/:id", get(get_resource_file).post(post_resource_file));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server on http://localhost:3030");
    axum::serve(listener, app).await.unwrap();
}

async fn get_books() -> Json<Vec<Book>> {
    Json(load_books())
}

async fn get_book(Path(isbn): Path<u64>) -> Json<Book> {
    Json(load_book(isbn))
}

async fn get_resources(Path(isbn): Path<u64>) -> Json<Vec<Resource>> {
    Json(load_resources(isbn))
}

async fn get_resource_file(Path(isbn): Path<u64>, Path(resource_id): Path<String>) -> Response<Body> {
    Response::new(Body::from(load_resource_file(isbn, resource_id)))
}

async fn post_book(Json(book): Json<Book>) -> (StatusCode, Json<Book>) {
    info!("Received book: {:?}", book);
    save_book(book.clone());
    (StatusCode::CREATED, Json(book))
}

async fn post_resources(Path(isbn): Path<u64>, Json(resource): Json<Resource>) -> (StatusCode, Json<Resource>) {
    info!("Received resource: {:?}", resource);
    if isbn != resource.book_isbn {
        return (StatusCode::BAD_REQUEST, Json(resource));
    }
    save_resource(resource.clone());
    (StatusCode::CREATED, Json(resource))
}

async fn post_resource_file(
    Path((isbn, resource_id)): Path<(u64, String)>,
    body: Body,
) -> (StatusCode, Json<String>) {
    let file_bytes = to_bytes(body, usize::MAX).await.unwrap();
    save_resource_file(isbn, resource_id.clone(), file_bytes.to_vec());
    (StatusCode::CREATED, Json(resource_id))
}

fn load_books() -> Vec<Book> {
    let books_json = fs::read_to_string("books.json").unwrap();
    serde_json::from_str(&books_json).unwrap()
}

fn load_book(isbn: u64) -> Book {
    let books = load_books();
    books.into_iter().find(|book| book.isbn == isbn).unwrap()
}

fn load_resources(isbn: u64) -> Vec<Resource> {
    let resources_dir = format!("{}", isbn);
    let mut resources = Vec::new();
    for entry in fs::read_dir(resources_dir).unwrap() {
        let path = entry.unwrap().path();
        let resource_json = fs::read_to_string(path).unwrap();
        let resource: Resource = serde_json::from_str(&resource_json).unwrap();
        resources.push(resource);
    }
    resources
}

fn load_resource_file(isbn: u64, resource_id: String) -> Vec<u8>{
    let resources_dir = format!("{}", isbn);
    let resource_path = format!("{}/{}.json", resources_dir, resource_id);
    if !FilePath::new(&resource_path).exists() {
        return Vec::new();
    }

    let resource_json = fs::read_to_string(resource_path).unwrap();
    let resource: Resource = serde_json::from_str(&resource_json).unwrap();

    // load the file at {resource_path}/{resource.file_name} and return it if it exists
    let file_path = format!("{}/{}", resources_dir, resource.file_name);
    if !FilePath::new(&file_path).exists() {
        return Vec::new();
    }

    fs::read(file_path).unwrap()
}

fn save_book(book: Book) {
    info!("Saving book: {:?}", book);
    let books = load_books();
    let mut books_map: HashMap<u64, Book> = books.into_iter().map(|b| (b.isbn, b)).collect();
    books_map.insert(book.isbn, book);
    let books_vec: Vec<Book> = books_map.into_values().collect();
    let books_json = serde_json::to_string(&books_vec).unwrap();
    fs::write("books.json", books_json).unwrap();
    info!("Book saved");
}

fn save_resource(resource: Resource) {
    let resources_dir = format!("{}", resource.book_isbn);
    if !FilePath::new(&resources_dir).exists() {
        fs::create_dir_all(&resources_dir).unwrap();
        info!("Created resources directory: {}", resources_dir);
    }
    let resource_json = serde_json::to_string(&resource).unwrap();
    let resource_path = format!("{}/{}.json", resources_dir, resource.id);
    fs::write(resource_path, resource_json).unwrap();
    info!("Saved resource: {:?}", resource);
}

fn save_resource_file(isbn: u64, resource_id: String, file: Vec<u8>) {
    let resources_dir = format!("{}", isbn);
    let resource_path = format!("{}/{}.json", resources_dir, resource_id);

    let resource_json = fs::read_to_string(resource_path).unwrap();
    let resource: Resource = serde_json::from_str(&resource_json).unwrap();

    let file_path = format!("{}/{}", resources_dir, resource.file_name);
    fs::write(file_path, file).unwrap();
}
