use crate::{
  glium::{backend::Context, program::Program},
  util::prelude::*,
};
use lazy_static::lazy_static;
use omt::gfx::ShaderLoader;
use regex::Regex;
use std::{
  collections::{BTreeMap, BTreeSet},
  rc::Rc,
};

lazy_static! {
  static ref IMPORT_REGEX: Regex =
    Regex::new(r##"^\s*#\s*import\s*"(?P<file>[\-\w.]+)"\s*$"##).unwrap();
}

struct ShaderSource {
  vertex: String,
  fragment: String,
}

#[derive(Default)]
pub struct ShaderSourceArchive {
  prefix: String,
  sources: BTreeMap<String, String>,
  shaders: BTreeMap<String, ShaderSource>,
}

impl ShaderSourceArchive {
  pub fn new(prefix: String) -> Self {
    Self {
      prefix,
      ..Default::default()
    }
  }
}

impl ShaderLoader for ShaderSourceArchive {
  fn register(&mut self, id: &str, src: &str) {
    self
      .sources
      .insert(format!("{}.{}", self.prefix, id), src.to_string());
  }

  fn register_shader(&mut self, id: &str, vertex_id: &str, fragment_id: &str) {
    self.shaders.insert(
      id.to_string(),
      ShaderSource {
        vertex: vertex_id.to_string(),
        fragment: fragment_id.to_string(),
      },
    );
  }
}

#[derive(Default)]
pub struct ShaderProgramArchive {
  sources: BTreeMap<String, String>,
  shaders: BTreeMap<String, ShaderSource>,
  programs: BTreeMap<String, Rc<Program>>,
}

impl ShaderProgramArchive {
  pub fn add_source_archive(&mut self, archive: ShaderSourceArchive) {
    for (id, source) in archive.sources {
      self.sources.insert(id, source);
    }

    for (id, shader_source) in archive.shaders {
      self.shaders.insert(id, shader_source);
    }
  }

  pub fn compile<L: Logger>(&mut self, logger: &L, ctx: Rc<Context>) {
    for (id, source) in &self.shaders {
      match Self::load_source(
        logger,
        &source.vertex,
        &self.sources,
        &mut Vec::default(),
        &mut BTreeSet::default(),
      ) {
        Ok(vertex) => match Self::load_source(
          logger,
          &source.fragment,
          &self.sources,
          &mut Vec::default(),
          &mut BTreeSet::default(),
        ) {
          Ok(fragment) => match Program::from_source(&ctx, &vertex, &fragment, None) {
            Ok(program) => {
              self.programs.insert(id.clone(), Rc::new(program));
            }
            Err(err) => {
              logger.error(format!("failed to load shader: {}", err));
            }
          },
          Err(err) => {
            logger.error(format!(
              "failed to load fragment shader for program {}: {}",
              id, err
            ));
          }
        },
        Err(err) => {
          logger.error(format!(
            "failed to load vertex shader for program {}: {}",
            id, err
          ));
        }
      }
    }
  }

  pub fn get(&self, id: &str) -> Option<Rc<Program>> {
    self.programs.get(id).cloned()
  }

  pub fn list(&self) -> Vec<String> {
    let mut ret = Vec::default();

    for id in self.shaders.keys() {
      ret.push(id.clone());
    }

    ret
  }

  fn load_source<'src, L: Logger>(
    logger: &L,
    source_id: &str,
    sources: &BTreeMap<String, String>,
    imported_files: &mut Vec<String>,
    successful_imports: &mut BTreeSet<String>,
  ) -> Result<String, String> {
    let source_code = sources.get(source_id).unwrap();

    // don't import self
    imported_files.push(source_id.to_string());

    let mut lines = Vec::new();
    for line in source_code.lines() {
      if let Some(caps) = IMPORT_REGEX.captures(line) {
        let import = caps["file"].to_string();

        if imported_files.contains(&import) {
          return Err(format!(
            "circular dependency detected processing: {} | this dependency requires current source: {}",
            source_id, import,
          ));
        }

        if successful_imports.contains(&import) {
          logger.debug(format!("already imported '{}', skipping", import));
          continue;
        }

        let import_source =
          Self::load_source(logger, &import, sources, imported_files, successful_imports)?;

        // replace import line with import source
        lines.push(import_source);

        successful_imports.insert(import);
      } else {
        lines.push(line.to_string());
      }
    }

    imported_files.pop();

    let code: String = lines.join("\n");

    Ok(code)
  }
}

impl AsPtr for ShaderProgramArchive {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn import_shader_works() {
    let test_strings = vec![r#"#import "main.shader.vs""#, r#"#import "main.shader.fs""#];
    let expected_imports = vec!["main.shader.vs", "main.shader.fs"];

    for (test, expected) in test_strings.iter().zip(expected_imports.iter()) {
      if let Some(caps) = IMPORT_REGEX.captures(test) {
        let import = caps["file"].to_string();
        assert_eq!(expected.to_string(), import);
      }
    }
  }
}
