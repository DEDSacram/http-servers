use warp::Filter;
use std::collections::HashMap;
#[tokio::main]
async fn main() {
    let map = HashMap::from([
        ("result", "Hello World"),
    ]);
    for val in map.values() {
        println!("{val}");
    }
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello")
        .map(move || warp::reply::json(&map));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 8080))
        .await;
}