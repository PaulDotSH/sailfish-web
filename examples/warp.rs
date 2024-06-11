use sailfish::TemplateOnce;
use sailfish_web::web_servers_support::template::TemplateWrapper;
use warp::Filter;
#[derive(Debug, TemplateOnce)]
#[template(path = "test.stpl")]
struct TestTemplate<'a> {
    s: &'a str,
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        // Sadly implementing .into() or .from() needs a huge name annotation, so this is the best alternative
        .map(|name: String| TemplateWrapper::from(TestTemplate { s: &name }).into_resp());

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
