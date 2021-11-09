fn main() {
  std::fs::remove_file("out.txt");
  shaders::preprocess_shaders();
}

fn log(msg: String) {
  use std::io::Write;
  std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open("out.txt")
    .unwrap()
    .write(msg.as_bytes());
}

mod shaders {
  use super::*;
  use lazy_static::lazy_static;
  use maplit::btreeset;
  use regex::Regex;
  use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
  };
  use toml::Value;
  use walkdir::WalkDir;

  lazy_static! {
    static ref SUPPORTED_SHADERS: BTreeSet<&'static str> = btreeset!["vertex", "fragment"];
    static ref CONFIG: PathBuf = PathBuf::new().join("assets").join("cfg").join("shaders");
    static ref SRC_DIR: PathBuf = PathBuf::new().join("assets").join("shaders").join("src");
    static ref OUT_DIR: PathBuf = PathBuf::new().join("assets").join("shaders").join("out");
    static ref IMPORT_REGEX: Regex =
      Regex::new(r##"^\s*#\s*import\s*"(?P<file>[\-\w.]+)"\s*$"##).unwrap();
  }

  #[derive(Debug)]
  struct ShaderSource {
    out: PathBuf,
    sources: BTreeSet<PathBuf>,
  }

  impl ShaderSource {
    /// Loads a shader and all its imports recursively
    /// Requires that 'src' points to the complete path
    fn load(source: &Path, output: &Path) -> Self {
      let mut source_dir = source.to_path_buf();
      source_dir.pop();
      match fs::read_to_string(source) {
        Ok(source_code) => {
          let source_code = fs::read_to_string(source).unwrap();
          let mut sources = BTreeSet::new();

          Self::determine_imports(&source_dir, source_code, &mut sources);

          sources.insert(source.to_path_buf());

          Self {
            out: output.to_path_buf(),
            sources,
          }
        }
        Err(e) => {
          panic!("could not find '{}'", source.display());
        }
      }
    }

    fn needs_update(&self) -> bool {
      if let Ok(out_metadata) = self.out.metadata() {
        let out_time = out_metadata.modified().unwrap();
        let mut needs_update = false;

        for source in &self.sources {
          match source.metadata() {
            Ok(source_metadata) => {
              let source_time = source_metadata.modified().unwrap();
              needs_update = source_time > out_time;
            }
            Err(e) => {
              panic!(
                "unable to find source file '{}', while checking for updates in '{}'",
                source.display(),
                self.out.display()
              );
            }
          }

          if needs_update {
            break;
          }
        }

        needs_update
      } else {
        true
      }
    }

    fn determine_imports(source_dir: &PathBuf, source: String, imports: &mut BTreeSet<PathBuf>) {
      for line in source.lines() {
        if let Some(caps) = IMPORT_REGEX.captures(line) {
          let import = source_dir.join(&caps["file"]);
          let mut import_dir = import.clone();
          import_dir.pop();
          match fs::read_to_string(&import) {
            Ok(import_source) => {
              imports.insert(import);
              Self::determine_imports(&import_dir, import_source, imports);
            }
            Err(e) => {
              panic!("could not find '{}'", import.display());
            }
          }
        }
      }
    }
  }

  fn push_line(line: String, lines: &mut Vec<String>, _output: &Path) {
    lines.push(line);
  }

  fn forge_shader(
    shader_path: &Path,
    output_path: &Path,
    imported_files: &mut Vec<String>,
    successful_imports: &mut BTreeSet<String>,
    out_lines: &mut Vec<String>,
    line_callback: fn(String, &mut Vec<String>, &Path),
  ) -> bool {
    let mut base_path = shader_path.to_path_buf();
    base_path.pop();

    let src = fs::read_to_string(shader_path).unwrap();

    let shader_str = shader_path.display();

    imported_files.push(format!("{}", shader_str));

    let mut lines = Vec::new();

    for line in src.lines() {
      if let Some(caps) = IMPORT_REGEX.captures(line) {
        let import = base_path.join(&caps["file"]);
        let import_str = format!("{}", import.display());

        if imported_files.contains(&import_str) {
          println!(
          "cargo:warning=circular dependency detected processing:\n{}\nalready imported file:\n{}",
          shader_str, import_str,
        );
        }

        if successful_imports.contains(&import_str) {
          println!("cargo:warning=skipping import {}", import_str);
          continue;
        }

        if !forge_shader(
          &import,
          output_path,
          imported_files,
          successful_imports,
          &mut lines,
          push_line,
        ) {
          return false;
        }

        successful_imports.insert(import_str);
      } else {
        lines.push(line.to_string());
      }
    }

    let code = lines.join("\n");

    line_callback(code, out_lines, output_path);

    imported_files.pop();

    true
  }

  fn write_file(code: String, lines: &mut Vec<String>, output: &Path) {
    let mut base_dir = output.to_path_buf();
    base_dir.pop();
    fs::create_dir_all(&base_dir).unwrap();

    lines.push(code);
    fs::write(output, lines.join("\n")).unwrap();
  }

  pub fn preprocess_shaders() {
    for entry in WalkDir::new(&*CONFIG) {
      let entry = entry.unwrap();
      if entry.file_type().is_file() {
        let data = fs::read_to_string(entry.path()).unwrap();
        let table = data.parse::<Value>().unwrap();
        let table = table.as_table().unwrap();

        for (_k, v) in table {
          let table = v.as_table().unwrap();
          for (k, v) in table {
            if SUPPORTED_SHADERS.contains(k.as_str()) {
              let filename = v.as_str().unwrap();
              let src_path = SRC_DIR.join(Path::new(filename));
              let out_path = OUT_DIR.join(Path::new(filename));

              let src_path_cpy = src_path.clone();
              let out_path_cpy = out_path.clone();

              let shader = ShaderSource::load(&src_path_cpy, &out_path_cpy);

              if shader.needs_update() {
                forge_shader(
                  &src_path,
                  &out_path,
                  &mut Vec::new(),
                  &mut BTreeSet::new(),
                  &mut Vec::new(),
                  write_file,
                );
              }
            }
          }
        }
      }
    }
  }
}
