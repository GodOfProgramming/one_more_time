use crate::util::prelude::*;
use omt::{
  core::{ShaderLoader, ShaderSource},
  glium::{
    backend::Context,
    program::{Program, ProgramCreationError},
  },
  lazy_static::lazy_static,
  regex::Regex,
  toml::Value,
};
use std::{
  collections::{BTreeMap, BTreeSet},
  fs,
  ops::Deref,
  path::{Path, PathBuf},
  rc::Rc,
};

lazy_static! {
  static ref IMPORT_REGEX: Regex =
    Regex::new(r##"^\s*#\s*import\s*"(?P<file>[\-\w.]+)"\s*$"##).unwrap();
}

#[derive(Default)]
pub struct ShaderSourceArchive {
  relative_path: PathBuf,
  sources: BTreeMap<String, ShaderSource>,
}

impl ShaderSourceArchive {
  pub fn new(relative_path: PathBuf) -> Self {
    Self {
      relative_path,
      sources: Default::default(),
    }
  }
}

impl ShaderLoader for ShaderSourceArchive {
  fn register(&mut self, id: &str, src: ShaderSource) {
    self.sources.insert(id.to_string(), src);
  }
}

struct ProgramSource {
  vertex: String,
  fragment: String,
}

pub struct Shader {
  program: Program,
}

impl Shader {
  fn from(ctx: Rc<Context>, sources: ProgramSource) -> Result<Self, ProgramCreationError> {
    let program = Program::from_source(&ctx, &sources.vertex, &sources.fragment, None)?;

    Ok(Self { program })
  }
}

impl Deref for Shader {
  type Target = Program;
  fn deref(&self) -> &Self::Target {
    &self.program
  }
}

#[derive(Default)]
pub struct ShaderProgramArchive {
  shaders: BTreeMap<String, Rc<Shader>>,
}

impl ShaderProgramArchive {
  pub fn add_source_archive<L: Logger>(
    &mut self,
    logger: &L,
    archive: ShaderSourceArchive,
    ctx: Rc<Context>,
  ) {
    let load_source = |file: &Path| {
      let src_path = archive.relative_path.join(file);

      Self::load_source(
        logger,
        &src_path,
        &mut Vec::default(),
        &mut BTreeSet::default(),
      )
    };

    for (id, source) in archive.sources {
      match load_source(&source.vertex) {
        Ok(vertex) => match load_source(&source.fragment) {
          Ok(fragment) => {
            let sources = ProgramSource { vertex, fragment };
            match Shader::from(ctx.clone(), sources) {
              Ok(shader) => {
                self.shaders.insert(id, Rc::new(shader));
              }
              Err(err) => {
                logger.error(format!("failed to load shader: {}", err));
              }
            }
          }
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

  pub fn get(&self, id: &str) -> Option<Rc<Shader>> {
    self.shaders.get(id).cloned()
  }

  pub fn list(&self) -> Vec<String> {
    let mut ret = Vec::default();

    for id in self.shaders.keys() {
      ret.push(id.clone());
    }

    ret
  }

  fn load_source<L: Logger>(
    logger: &L,
    shader_path: &Path,
    imported_files: &mut Vec<String>,
    successful_imports: &mut BTreeSet<String>,
  ) -> Result<String, String> {
    let mut base_path = shader_path.to_path_buf();
    base_path.pop();

    let source_code = fs::read_to_string(shader_path).unwrap();
    let shader_str = shader_path.to_str().unwrap();

    // don't import self
    imported_files.push(shader_str.to_string());

    let mut lines = Vec::new();

    for line in source_code.lines() {
      if let Some(caps) = IMPORT_REGEX.captures(line) {
        let import = base_path.join(&caps["file"]);
        let import_str = format!("{}", import.display());

        if imported_files.contains(&import_str) {
          return Err(format!(
            "circular dependency detected processing:\n{}\nalready imported file:\n{}",
            shader_str, import_str,
          ));
        }

        if successful_imports.contains(&import_str) {
          logger.debug(format!("already imported '{}', skipping", import_str));
          continue;
        }

        let import_source = Self::load_source(logger, &import, imported_files, successful_imports)?;

        // replace import line with import source
        lines.push(import_source);

        successful_imports.insert(import_str);
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
