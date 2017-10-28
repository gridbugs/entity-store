extern crate itertools;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;

mod storage_type;
mod aggregate_type;
mod spec;
mod result;
mod input;
mod output;
mod renderer;
mod code_gen;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use result::{GenResult, SaveResult, SaveError};
use code_gen::CodeGen;

pub struct GeneratedCode {
    text: String,
}

fn combine_modules(m: &Vec<(String, String)>) -> String {
    let module_text = m.iter().map(|&(ref name, ref contents)| {
        if name == "mod" {
            contents.clone()
        } else {
            let indented = itertools::join(
                contents.split("\n").map(|s| {
                    if s == "" {
                        "".to_string()
                    } else {
                        format!("    {}", s)
                    }
                }), "\n");
            format!("mod {} {{\n{}}}", name, indented)
        }
    });
    itertools::join(module_text, "\n\n")
}



impl GeneratedCode {
    pub fn generate(s: &str) -> GenResult<Self> {
        let code_gen = CodeGen::new(s)?;
        let modules = code_gen.render()?;
        let text = combine_modules(&modules);

        Ok(Self {
            text,
        })
    }

    pub fn save(&self) -> SaveResult<()> {
        let out_dir = env::var("OUT_DIR")
            .map_err(|e| SaveError::VarError(e, "This method must be called from a build script."))?;

        let dest_path = Path::new(&out_dir).join("mod.rs");
        let mut file = File::create(&dest_path)
            .map_err(|e| SaveError::FailedToCreateFile(dest_path.clone(), e))?;
        file.write_all(self.text.as_bytes())
            .map_err(|e| SaveError::FailedToWriteFile(dest_path.clone(), e))?;

        Ok(())
    }
}
