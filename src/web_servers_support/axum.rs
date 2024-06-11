use sailfish::TemplateOnce;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use crate::web_servers_support::template::TemplateWrapper;

impl<T> IntoResponse for TemplateWrapper<T>
    where
        T: TemplateOnce,
{
    fn into_response(self) -> Response {
        match self.template.render_once() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                let error_message = self.custom_error_message.unwrap_or_else(|| {
                    format!("Failed to render template. Error: {err}")
                });
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}