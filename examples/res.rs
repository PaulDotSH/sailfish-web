// ```test2.stpl
// <h1>Hello <b><%- s %></b></h1>
// ```

use sailfish::TemplateOnce;
use sailfish_web::option_result::Res;

#[derive(TemplateOnce)]
#[template(path = "test2.stpl")]
struct TestTemplate {
    s: Res<String, i32>,
}

fn main() {
    print!(
        "{}\n",
        TestTemplate {
            s: Ok("world".to_string()).into()
        }
        .render_once()
        .unwrap()
    );
    print!(
        "{}",
        TestTemplate { s: Err(123).into() }.render_once().unwrap()
    )
}

// Output
// <h1>Hello <b>world</b></h1>
// <h1>Hello <b>Error: 123</b></h1>
