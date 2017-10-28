use tera::Tera;
use result::GenResult as Result;
use result::GenError as Error;
use output::Spec;

pub struct Renderer {
    tera: Tera,
    module_names: Vec<&'static str>,
}

impl Renderer {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();

        let templates = vec![
            ("mod",                     include_str!("templates/mod.template.rs")),
            ("id",                      include_str!("templates/id.template.rs")),
            ("component",               include_str!("templates/component.template.rs")),
            ("entity_vec",              include_str!("templates/entity_vec.template.rs")),
            ("entity_store",            include_str!("templates/entity_store.template.rs")),
            ("entity_change",           include_str!("templates/entity_change.template.rs")),
            ("component_type_set",      include_str!("templates/component_type_set.template.rs")),
            ("constants",               include_str!("templates/constants.template.rs")),
            ("entity_component_table",  include_str!("templates/entity_component_table.template.rs")),
        ];

        let module_names = templates.iter().map(|&(n, _)| n).collect();

        tera.add_raw_templates(templates)?;

        Ok(Renderer {
            tera,
            module_names,
        })
    }

    pub fn render(&self, spec: &Spec) -> Result<Vec<(String, String)>> {
        self.module_names.iter().map(|module_name| {
            self.tera.render(module_name, spec)
                .map_err(Error::TemplateError)
                .map(|s| (module_name.to_string(), s))
        }).collect()
    }
}
