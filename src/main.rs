use std::net::SocketAddr;

use axum::{
    debug_handler,
    extract::Path,
    Router, routing::post,
};
pub struct Web {}

impl Web {
    #[debug_handler]
    pub async fn create_event(Path(foo): Path<String>,
                              payload: String)
                              -> Result<String, String> {
        Ok(format!("Foo: {foo}, Payload: {payload:?}"))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/events/:foo/", post(Web::create_event));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
