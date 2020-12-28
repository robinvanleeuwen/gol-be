use warp::Filter;
use golbe::{new_run_count, establish_connection};
use serde_json;
use serde_json::Value;

fn insert_new_runcount(run: i32, count: i32) -> String {
    let conn = establish_connection();
    let result = new_run_count(&conn, &run, &count);
    let json: Value;

    match result {
        Ok(x) => {
            json = serde_json::json!(x);
        },
        _ => {
            json = serde_json::json!(r#"{ "error":"100" }"#);
        }
    }

    format!("{}", json)
}

#[tokio::main]
async fn main() {
    // GET /hello/warp

    let hello = warp::path!("hello" / String)
        .map(|name| {
           format!("Hello {}", name)
        });

    let bye = warp::path!("bye")
        .map(||{
            format!("Goodbye World!")
        });

    let runcount = warp::path!("runcount" / i32 / i32)
        .map(|run,count|{
           insert_new_runcount(run, count)
        });

    let routes = warp::get()
        .and(
        hello
            .or(bye)
            .or(runcount)
        );

    warp::serve(routes)
        .run(([127,0,0,1], 3000))
        .await
}
