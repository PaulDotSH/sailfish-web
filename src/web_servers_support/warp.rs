use sailfish::TemplateOnce;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;
use crate::web_servers_support::template::TemplateWrapper;

impl<T: TemplateOnce> TemplateWrapper<T> {
    pub fn into_resp(self) -> Response {
        match self.template.render_once() {
            Ok(html) => Response::new(Body::from(html)),
            Err(err) => {
                let error_message = self.custom_error_message.unwrap_or_else(|| {
                    format!("Failed to render template. Error: {err}")
                });
                let mut response = Response::new(Body::from(error_message));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response            }
        }
    }
}
