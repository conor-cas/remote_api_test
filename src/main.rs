use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello from fly.io");

    let post_endpoint = warp::path("post")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            format!("Received: {}", body)
        });

    let routes = hello.or(post_endpoint);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}