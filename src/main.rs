use axum::{
    extract,
    http::StatusCode,
    routing::{get, post},
    Router,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/hello/:username", get(hello_handler));
    axum::Server::bind(&"0.0.0.0:57578".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn hello_handler(extract::Path(username): extract::Path<String>) -> String {
    format!("hello, {}!", username)
}
