extern crate itertools;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate rustfmt;

mod storage_type;
mod aggregate_type;
mod spec;
mod result;
mod input;
mod output;
mod renderer;
mod code_gen;

use std::env;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

use result::{GenResult, SaveResult, SaveError};
use code_gen::CodeGen;

pub struct GeneratedCode {
    files: BTreeMap<PathBuf, String>,
}

impl GeneratedCode {
    pub fn generate(s: &str) -> GenResult<Self> {
        let code_gen = CodeGen::new(s)?;

        Ok(Self {
            files: code_gen.render()?,
        })
    }

    pub fn save(&self) -> SaveResult<()> {
        let out_dir = env::var("OUT_DIR")
            .map_err(|e| SaveError::VarError(e, "This method must be called from a build script."))?;

        for (filename, contents) in self.files.iter() {
            let dest_path = Path::new(&out_dir).join(filename);
            let mut file = File::create(&dest_path)
                .map_err(|e| SaveError::FailedToCreateFile(dest_path.clone(), e))?;
            file.write_all(contents.as_bytes())
                .map_err(|e| SaveError::FailedToWriteFile(dest_path.clone(), e))?;
        }

        Ok(())
    }
}
