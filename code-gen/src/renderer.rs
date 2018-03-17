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
            ("iterators",               include_str!("templates/iterators.template.rs")),
            ("entity_store",            include_str!("templates/entity_store.template.rs")),
            ("entity_store_raw",        include_str!("templates/entity_store_raw.template.rs")),
            ("flat_collections",        include_str!("templates/flat_collections.template.rs")),
            ("vec_collections",         include_str!("templates/vec_collections.template.rs")),
            ("spatial_hash",            include_str!("templates/spatial_hash.template.rs")),
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
