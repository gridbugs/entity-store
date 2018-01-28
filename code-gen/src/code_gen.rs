use result::GenResult as Result;
use spec::Spec;
use output;
use renderer::Renderer;

pub struct CodeGen {
    spec: output::Spec,
    renderer: Renderer,
}

impl CodeGen {
    pub fn new(spec_toml: &str) -> Result<Self> {
        let spec = Spec::from_str(spec_toml)?;
        let spec = spec.to_output();

        let renderer = Renderer::new()?;

        Ok(Self {
            spec,
            renderer,
        })
    }

    pub fn render(&self) -> Result<Vec<(String, String)>> {
        self.renderer.render(&self.spec)
    }
}
