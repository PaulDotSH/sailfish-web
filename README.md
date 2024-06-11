# Sailfish-Web
Simple extension crate that enables a simpler api for web server crates

# Examples

```rust
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
```

For more examples you can check the [examples](https://github.com/pauldotsh/sailfish-web/tree/master/examples).
