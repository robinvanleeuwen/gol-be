use warp::Filter;

mod lib;

#[tokio::main]
async fn main() {
    // GET /hello/warp

    let hello = warp::path!("hello" / String)
        .map(|name| {
           format!("Hello {}", name)
        });

    warp::serve(hello)
        .run(([127,0,0,1], 3000))
        .await
}
