use axum::{response::IntoResponse, routing::get, Router};
use sailfish::TemplateOnce;
use sailfish_web::web_servers_support::template::TemplateWrapper;
use tokio::net::TcpListener;

#[derive(TemplateOnce)]
#[template(path = "test.stpl")]
struct TestTemplate<'a> {
    s: &'a str,
}

async fn endpoint_1() -> impl IntoResponse {
    TemplateWrapper::from(TestTemplate { s: "test" })
        .with_custom_error_message("<html><b>Error message</b></html>".to_string())
}

async fn endpoint_2() -> impl IntoResponse {
    TemplateWrapper::from(TestTemplate { s: "test" })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/e1", get(endpoint_1))
        .route("/e2", get(endpoint_2));

    let listener = TcpListener::bind("127.0.0.1:1312")
        .await
        .expect("Cannot start server");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}