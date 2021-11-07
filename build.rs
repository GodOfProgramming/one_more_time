fn main() {
  shaders::preprocess_shaders();
}

mod shaders {
  use maplit::btreeset;
  use regex::Regex;
  use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
  };
  use toml::Value;
  use walkdir::WalkDir;

  fn push_line(line: String, lines: &mut Vec<String>, _output: &Path) {
    lines.push(line);
  }

  fn forge_shader(
    import_regex: &Regex,
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
      if let Some(caps) = import_regex.captures(line) {
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
          import_regex,
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
    lines.push(code);
    fs::write(output, lines.join("\n")).unwrap();
  }

  pub fn preprocess_shaders() {
    let supported_shaders = btreeset!["vertex", "fragment"];
    let config = PathBuf::new().join("assets").join("cfg").join("shaders");

    let src_dir = PathBuf::new().join("assets").join("shaders").join("src");
    let out_dir = PathBuf::new().join("assets").join("shaders").join("out");

    let import_regex = Regex::new(r##"^\s*#\s*import\s*"(?P<file>[\-\w.]+)"\s*$"##).unwrap();

    fs::create_dir_all(&out_dir).unwrap();

    for entry in WalkDir::new(config) {
      let entry = entry.unwrap();
      if entry.file_type().is_file() {
        let data = fs::read_to_string(entry.path()).unwrap();
        let table = data.parse::<Value>().unwrap();
        let table = table.as_table().unwrap();

        for (_k, v) in table {
          let table = v.as_table().unwrap();
          for (k, v) in table {
            if supported_shaders.contains(k.as_str()) {
              let filename = v.as_str().unwrap();
              let src_path = src_dir.join(filename);
              let out_path = out_dir.join(filename);

              let src_meta = src_path.metadata().unwrap();
              let out_meta = out_path.metadata();

              let should_forge = if let Ok(out_meta) = out_meta {
                let src_time = src_meta.modified().unwrap();
                let out_time = out_meta.modified().unwrap();
                src_time > out_time
              } else {
                true
              };

              if should_forge {
                forge_shader(
                  &import_regex,
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
