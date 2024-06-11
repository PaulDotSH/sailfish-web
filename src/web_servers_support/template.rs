use sailfish::TemplateOnce;

/// Wrapper struct for TemplateOnce that allows us to have custom error messages and a better api for using the web frameworks
pub struct TemplateWrapper<T: TemplateOnce> {
    pub(crate) template: T,
    pub(crate) custom_error_message: Option<String>,
}

impl<T> From<T> for TemplateWrapper<T>
    where
        T: TemplateOnce,
{
    fn from(template: T) -> Self {
        TemplateWrapper { template, custom_error_message: None }
    }
}

impl<T: TemplateOnce> TemplateWrapper<T> {
    /// Create an AxumTemplate from a TemplateOnce struct
    pub fn from(template: T) -> Self {
        Self {
            template,
            custom_error_message: None,
        }
    }

    /// Specify a custom error message to be rendered when template.render_once() returns an RenderError
    pub fn with_custom_error_message(mut self, message: String) -> Self {
        self.custom_error_message = Some(message);
        self
    }
}