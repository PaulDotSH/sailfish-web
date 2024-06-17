// ```test2.stpl
// <h1>Hello <b><%- s %></b></h1>
// ```

use sailfish::TemplateOnce;
use sailfish_web::option_result::Opt;

#[derive(TemplateOnce)]
#[template(path = "test2.stpl")]
struct TestTemplate {
    s: Opt<String>,
}

fn main() {
    print!(
        "{}\n",
        TestTemplate {
            s: Some("world".to_string()).into()
        }
            .render_once()
            .unwrap()
    );
    print!("{}", TestTemplate { s: None.into() }.render_once().unwrap())
}

// Output
// <h1>Hello <b>world</b></h1>
// <h1>Hello <b></b></h1>
