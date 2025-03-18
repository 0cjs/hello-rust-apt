use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello").map(|| // route
        warp::reply::with_status("Hello.\n", warp::http::StatusCode::OK));
    warp::serve(hello)
        .run(([127, 0, 0, 1], 9999)).await;
}
