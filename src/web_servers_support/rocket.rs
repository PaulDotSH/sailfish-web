use rocket::Response;
use rocket::response::content::RawHtml;
use sailfish::TemplateOnce;
use crate::web_servers_support::template::TemplateWrapper;

impl<T: TemplateOnce> Into<RawHtml<String>> for TemplateWrapper<T> {
    fn into(self) -> RawHtml<String> {
        match self.template.render_once() {
            Ok(html) => RawHtml(html),
            Err(err) => {
                let error_message = self.custom_error_message.unwrap_or_else(|| {
                    format!("Failed to render template. Error: {err}")
                });
                RawHtml(error_message)
            }
        }
    }
}