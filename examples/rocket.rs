use rocket::get;
use rocket::launch;
use rocket::routes;
use sailfish::TemplateOnce;
use sailfish_web::web_servers_support::template::TemplateWrapper;
use rocket::response::content::RawHtml;
use rocket::Response;

#[derive(TemplateOnce)]
#[template(path = "test.stpl")]
struct TestTemplate<'a> {
    s: &'a str,
}

#[get("/hello")]
fn hello() -> RawHtml<String> {
    TemplateWrapper::from(TestTemplate { s: "test" }).into()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}