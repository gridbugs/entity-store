//! Code generator for a game data store.
//! Data is organised into "components" - eaching having
//! a particular rust type. "Entities" - objects in the
//! game world - are collections of components. A
//! component can be associated with an entity by storing
//! the id of the entity in that component's store.
//!
//! A simple example:
//!
//! ```
//! struct EntityStore {
//!     position: HashMap<EntityId, ::cgmath::Vector2<f32>>,
//!     solid: HashSet<EntityId>,
//!     tile: HashSet<EntityId, MyTileType>,
//! }
//! ```
//!
//! Note the `solid` field is a `HashSet` rather than a
//! `HashMap`. Sets are used to store flags with no
//! associated data.
//!
//! This must be used from a build script. A simple build
//! script looks like:
//!
//! ```
//! extern crate entity_store_code_gen;
//!
//! fn main() {
//!     entity_store_code_gen::generate(include_str!("spec.toml"), "entity_store.rs").unwrap()
//! }
//! ```
//!
//! Use [entity_store_helper](https://crates.io/crates/entity_store_helper)
//! to help make use of the generated code.
#[macro_use] extern crate itertools;
extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;
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

pub use result::{GenResult, GenError, SaveResult, SaveError};
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

    pub fn save(&self, filename: &str) -> SaveResult<()> {
        let env_error_str = "This method must be called from a build script.";
        let out_dir = env::var("OUT_DIR")
            .map_err(|e| SaveError::VarError(e, env_error_str))?;

        let dest_path = Path::new(&out_dir).join(filename);
        let mut file = File::create(&dest_path)
            .map_err(|e| SaveError::FailedToCreateFile(dest_path.clone(), e))?;
        file.write_all(self.text.as_bytes())
            .map_err(|e| SaveError::FailedToWriteFile(dest_path.clone(), e))?;

        #[cfg(unix)]
        {
            // make a symlink to the generated file
            let manifest_dir = env::var("CARGO_MANIFEST_DIR")
                .map_err(|e| SaveError::VarError(e, env_error_str))?;
            let code_gen_symlink_path = Path::new(&manifest_dir).join(".generated.rs");
            if code_gen_symlink_path.exists() {
                ::std::fs::remove_file(&code_gen_symlink_path)
                    .map_err(|e| SaveError::FailedToRemoveExistingSymlink(code_gen_symlink_path.to_path_buf(), e))?;
            }
            ::std::os::unix::fs::symlink(&dest_path, code_gen_symlink_path)
                .map_err(|e| SaveError::FailedToMakeSymlink(dest_path.to_path_buf(), e))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Gen(GenError),
    Save(SaveError),
}

/// Generates code from a given toml spec.
/// Results are placed in OUT_DIR.
/// Must be called from a build script.
pub fn generate(spec: &str, filename: &str) -> Result<(), Error> {
    let code = GeneratedCode::generate(spec).map_err(Error::Gen)?;
    code.save(filename).map_err(Error::Save)
}

#[macro_export]
macro_rules! generate_entity_store {
    ($spec:expr, $filename:expr) => {
        ::entity_store_code_gen::generate(include_str!($spec), $filename).unwrap()
    }
}
