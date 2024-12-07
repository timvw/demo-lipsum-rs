use axum::extract::Query;
use axum::{response::Json, routing::get, Router};
use serde::Deserialize;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(lipsum));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct LipsumRequestParams {
    #[serde(default = "num_words_default")]
    num_words: usize,
}

fn num_words_default() -> usize {
    100
}

async fn lipsum(q: Query<LipsumRequestParams>) -> Json<Value> {
    let content = lipsum::lipsum(q.num_words);
    Json(json!({ "text": content }))
}
