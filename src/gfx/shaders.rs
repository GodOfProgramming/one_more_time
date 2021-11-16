use crate::util::{self, DirID};
use glium::program::{Program, ProgramCreationError, ShaderType};
use lazy_static::lazy_static;
use log::{error, info, warn};
use regex::Regex;
use std::{
  collections::{BTreeMap, BTreeSet},
  fs,
  path::{Path, PathBuf},
};
use toml::Value;

lazy_static! {
  static ref SRC_DIR: PathBuf = PathBuf::new().join("assets").join("shaders").join("src");
  static ref IMPORT_REGEX: Regex =
    Regex::new(r##"^\s*#\s*import\s*"(?P<file>[\-\w.]+)"\s*$"##).unwrap();
}

#[derive(Debug)]
struct ShaderSource {
  out: PathBuf,
  sources: BTreeSet<PathBuf>,
}

impl ShaderSource {
  fn load_source(
    shader_path: &Path,
    imported_files: &mut Vec<String>,
    successful_imports: &mut BTreeSet<String>,
  ) -> Result<String, String> {
    let mut base_path = shader_path.to_path_buf();
    base_path.pop();

    let source_code = fs::read_to_string(shader_path).unwrap();
    let shader_str = shader_path.display();

    imported_files.push(format!("{}", shader_str));

    let mut lines = Vec::new();

    for line in source_code.lines() {
      if let Some(caps) = IMPORT_REGEX.captures(line) {
        let import = base_path.join(&caps["file"]);
        let import_str = format!("{}", import.display());

        if imported_files.contains(&import_str) {
          return Err(format!(
          "cargo:warning=circular dependency detected processing:\n{}\nalready imported file:\n{}",
          shader_str, import_str,
        ));
        }

        if successful_imports.contains(&import_str) {
          warn!("cargo:warning=already imported '{}', skipping", import_str);
          continue;
        }

        let import_source = Self::load_source(&import, imported_files, successful_imports)?;
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

#[derive(Default)]
pub struct ProgramSources {
  vertex: String,
  fragment: String,
}

impl From<PotentialProgramSources> for ProgramSources {
  fn from(sources: PotentialProgramSources) -> Self {
    Self {
      vertex: sources.vertex.unwrap(),
      fragment: sources.fragment.unwrap(),
    }
  }
}

#[derive(Default)]
struct PotentialProgramSources {
  vertex: Option<String>,
  fragment: Option<String>,
}

impl PotentialProgramSources {
  fn is_valid(&self) -> bool {
    self.vertex.is_some() && self.fragment.is_some()
  }
}

pub struct ShaderSources {
  sources: BTreeMap<DirID, ProgramSources>,
}

impl ShaderSources {
  pub fn new() -> Self {
    Self {
      sources: BTreeMap::default(),
    }
  }

  pub fn load_all(&mut self) {
    let config = PathBuf::new().join("assets").join("cfg").join("shaders");
    util::iterate_dir_with_id(&config, |path, id| {
      let data = fs::read_to_string(path)
        .map_err(|e| format!("cannot find {}, err = {}", path.display(), e))
        .unwrap();
      let table = data.parse::<Value>().unwrap();
      let table = table.as_table().unwrap();

      for (local_shader_id, shaders) in table {
        let new_id = id.extend(&local_shader_id);
        let shaders = shaders.as_table().unwrap();

        let mut sources = PotentialProgramSources::default();

        for (shader_type, filename) in shaders {
          let current_source = match shader_type.as_str() {
            "vertex" => &mut sources.vertex,
            "fragment" => &mut sources.fragment,
            invalid => {
              warn!("unsupported shader type: {}", invalid);
              continue;
            }
          };

          if let Value::String(filename) = filename {
            let src_path = SRC_DIR.join(Path::new(filename));
            match ShaderSource::load_source(
              &src_path,
              &mut Vec::default(),
              &mut BTreeSet::default(),
            ) {
              Ok(source) => {
                *current_source = Some(source);
              }
              Err(msg) => {
                error!("{}", msg);
                continue;
              }
            }
          } else {
            error!("shader path is not a string");
            continue;
          }
        }

        if sources.is_valid() {
          self.sources.insert(new_id, sources.into());
        } else {
          error!("required shader types not present for program");
        }
      }
    });
  }

  pub fn load_repository(self, ctx: &std::rc::Rc<glium::backend::Context>) -> ShaderRepository {
    let mut repo = ShaderRepository::default();

    for (id, sources) in self.sources {
      match Shader::from(ctx.clone(), sources) {
        Ok(shader) => {
          repo.shaders.insert(id, shader);
        }
        Err(msg) => {
          error!("cannot load shader {}", msg);
        }
      }
    }

    repo
  }
}

pub struct Shader {
  program: Program,
}

impl Shader {
  fn from(
    ctx: std::rc::Rc<glium::backend::Context>,
    sources: ProgramSources,
  ) -> Result<Self, ProgramCreationError> {
    let program = Program::from_source(&ctx, &sources.vertex, &sources.fragment, None)?;

    Ok(Self { program })
  }
}

#[derive(Default)]
pub struct ShaderRepository {
  shaders: BTreeMap<DirID, Shader>,
}

impl ShaderRepository {}
