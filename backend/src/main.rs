pub mod books;
pub mod app;

use warp::Filter;

#[tokio::main]
async fn main() {
    let apiV = warp::path!("api").map(|| "api version 0.1");
    let cdnV = warp::path!("cdn").map(|| "cdn version 0.1");
    let app = App::init();

    warp::serve(apiV)
        .run(([127, 0, 0, 1], 3030))
        .await;

    app.deinit();
}
