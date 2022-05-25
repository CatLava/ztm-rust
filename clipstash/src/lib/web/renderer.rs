use crate::web::ctx;

#[derive(Debug, thiserror::Error)]
pub enum RenderError{
    Render(#[from] handlebars::RenderError)
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>)

impl<'a> Renderer<'a> {
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs", &template_dir);
            // this returns a result and will handle with a .expect
            .expect("failed to register handlebars templates");
        Self(renderer)
    }

    fn convert_to_value<S>(serializable: &S) -> serde_json::Value 
    where 
        S: serde::Serialize + std::fmt::Debug,
        {
            // this converts everything into a value before browser
            serde_json::to_value(&serializable)
                .expect("failed to convert structure to value")
        }
}