use std::collections::BTreeMap;
use std::path::PathBuf;
use tera::Tera;
use rustfmt;
use result::GenResult as Result;
use result::GenError as Error;
use output::Spec;

pub struct Renderer {
    tera: Tera,
    filenames: Vec<&'static str>,
}

impl Renderer {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();

        let templates = vec![
            ("mod.rs",              include_str!("templates/mod.template.rs")),
            ("id.rs",               include_str!("templates/id.template.rs")),
            ("component.rs",        include_str!("templates/component.template.rs")),
            ("entity_vec.rs",       include_str!("templates/entity_vec.template.rs")),
            ("entity_store.rs",     include_str!("templates/entity_store.template.rs")),
            ("entity_change.rs",    include_str!("templates/entity_change.template.rs")),
        ];

        let filenames = templates.iter().map(|&(n, _)| n).collect();

        tera.add_raw_templates(templates)?;

        Ok(Renderer {
            tera,
            filenames,
        })
    }

    pub fn render(&self, spec: &Spec) -> Result<BTreeMap<PathBuf, String>> {
        self.filenames.iter().map(|filename| {
            self.tera.render(filename, spec)
                .map_err(Error::TemplateError)
                .and_then(|s| {

                    let mut b = s.clone().into_bytes();

                    rustfmt::format_input(
                        rustfmt::Input::Text(s),
                        &rustfmt::config::Config::default(),
                        Some(&mut b)).map_err(|_| Error::RustFmtError)?;

                    let s = String::from_utf8(b).map_err(|_| Error::Utf8ConversionError)?;

                    Ok((PathBuf::from(filename), s))
                })
        }).collect()
    }
}
