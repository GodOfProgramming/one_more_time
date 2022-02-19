use crate::input::keyboard::Key;

type ConversionResult<T> = Result<T, String>;

pub mod string {
  use super::*;
  use crate::math::glm::IVec2;
  use log::warn;
  use std::{ffi::CString, num::NonZeroU8};

  pub fn into_cstring(s: &str) -> CString {
    CString::from(
      s.as_bytes()
        .iter()
        .map(|b| NonZeroU8::new(*b).unwrap())
        .collect::<Vec<NonZeroU8>>(),
    )
  }

  /**
   * String must be of the form "(x, y)" with whitespace a non-issue
   */
  pub fn into_vec2(s: &str) -> ConversionResult<IVec2> {
    let expect = |buff: &[u8], index: &mut usize, expected_char| {
      if let Some(c) = buff.get(*index) {
        let c = *c as char;
        *index += 1;

        if c == expected_char {
          return true;
        }
      }

      false
    };

    let parse_num = |buff: &[u8], index: &mut usize| {
      let mut x_parts = Vec::new();

      while let Some(c) = buff.get(*index) {
        let c = *c as char;
        if !('0'..='9').contains(&c) {
          break;
        }

        *index += 1;

        x_parts.push(c);
      }

      let s: String = x_parts.into_iter().collect();

      if let Ok(v) = s.parse::<i32>() {
        Ok(v)
      } else {
        Err(())
      }
    };

    let skip_whitespace = |buff: &[u8], index: &mut usize| {
      while let Some(c) = buff.get(*index) {
        let c = *c as char;
        if c != ' ' && c != '\n' && c != '\t' && c != '\r' {
          break;
        }

        *index += 1;
      }
    };

    let mut vec = IVec2::default();

    let mut index = 0;
    let buff = s.as_bytes();

    skip_whitespace(buff, &mut index);

    if !expect(buff, &mut index, '(') {
      return Err(String::from("expected '('"));
    }

    skip_whitespace(buff, &mut index);

    if let Ok(x) = parse_num(buff, &mut index) {
      vec.x = x;
    } else {
      return Err(String::from("expected number"));
    }

    skip_whitespace(buff, &mut index);

    if !expect(buff, &mut index, ',') {
      return Err(String::from("expected ','"));
    }

    skip_whitespace(buff, &mut index);

    if let Ok(y) = parse_num(buff, &mut index) {
      vec.y = y;
    } else {
      return Err(String::from("expected number"));
    }

    skip_whitespace(buff, &mut index);

    if !expect(buff, &mut index, ')') {
      return Err(String::from("expected ')'"));
    }

    Ok(vec)
  }
}

pub mod path {
  use lazy_static::lazy_static;
  use regex::Regex;
  use std::path::{Component, Path, PathBuf};

  pub fn into_id(path: &Path) -> String {
    let mut path = PathBuf::from(path);
    path.set_extension("");

    let mut vec = path
      .components()
      .map(|c| {
        let mut s = c.as_os_str().to_str().unwrap_or_default().to_string();

        if cfg!(windows) {
          lazy_static! {
            static ref WIN_ROOT_RE: Regex = Regex::new(r"(?P<drive>[A-Z]):").unwrap();
          }

          if let Some(cap) = WIN_ROOT_RE.captures(&s) {
            s = cap["drive"].to_string();
          }
        }

        s
      })
      .collect::<Vec<String>>();

    vec.retain(|v| v != "\\");

    if path.has_root() {
      vec.insert(0, "".to_string());
    }

    vec.join(".")
  }
}

pub mod value {
  use super::*;
  use toml::Value;

  pub fn into_u8(value: &Value) -> ConversionResult<u8> {
    if let Value::Integer(v) = value {
      Ok(*v as u8)
    } else {
      Err(String::from("value was not an integer (u8)"))
    }
  }

  pub fn into_u32(value: &Value) -> ConversionResult<u32> {
    if let Value::Integer(v) = value {
      Ok(*v as u32)
    } else {
      Err(String::from("value was not an integer (u32)"))
    }
  }

  pub fn into_f32(value: &Value) -> ConversionResult<f32> {
    if let Value::Float(v) = value {
      Ok(*v as f32)
    } else {
      Err(String::from("value was not a float (f32)"))
    }
  }

  pub fn into_str(value: &Value) -> ConversionResult<String> {
    if let Value::String(v) = value {
      Ok(v.clone())
    } else {
      Err(String::from("value was not a string"))
    }
  }

  pub fn into_vec_str(value: &Value) -> ConversionResult<Vec<String>> {
    if let Value::Array(array) = value {
      let mut storage = Vec::new();
      for item in array {
        storage.push(into_str(item)?);
      }
      Ok(storage)
    } else {
      Err(String::from("value was not an array of strings"))
    }
  }

  pub fn into_key(value: &str) -> ConversionResult<Key> {
    Ok(match value {
      "a" => Key::A,
      "b" => Key::B,
      "c" => Key::C,
      "d" => Key::D,
      "e" => Key::E,
      "f" => Key::F,
      "g" => Key::G,
      "h" => Key::H,
      "i" => Key::I,
      "j" => Key::J,
      "k" => Key::K,
      "l" => Key::L,
      "m" => Key::M,
      "n" => Key::N,
      "o" => Key::O,
      "p" => Key::P,
      "q" => Key::Q,
      "r" => Key::R,
      "s" => Key::S,
      "t" => Key::T,
      "u" => Key::U,
      "v" => Key::V,
      "w" => Key::W,
      "x" => Key::X,
      "y" => Key::Y,
      "z" => Key::Z,
      "esc" => Key::Escape,
      "space" => Key::Space,
      "up" => Key::UpArrow,
      "down" => Key::DownArrow,
      "left" => Key::LeftArrow,
      "right" => Key::RightArrow,
      _ => return Err(format!("key '{}' does not have string mapping", value)),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::{path, string};
  use crate::math::glm::IVec2;
  use std::path::Path;

  #[test]
  fn string_parse_vec2() {
    let tests = vec![
      (" (1,2)", Ok(IVec2::new(1, 2))),
      (" (1, 2)", Ok(IVec2::new(1, 2))),
      ("(1, 2) ", Ok(IVec2::new(1, 2))),
      ("(1, 2)", Ok(IVec2::new(1, 2))),
      ("(12, 34)", Ok(IVec2::new(12, 34))),
      ("(123, 456)", Ok(IVec2::new(123, 456))),
      ("(1,2)", Ok(IVec2::new(1, 2))),
      ("1, 23)", Err(())),
      ("(1, 23", Err(())),
      ("1, 2", Err(())),
      ("()", Err(())),
      ("(1 2)", Err(())),
      (",", Err(())),
    ];

    for (s, r) in tests {
      assert_eq!(r, string::into_vec2(s));
    }
  }

  #[test]
  fn path_parse_id() {
    let tests = vec![
      ("some/path", "some.path"),
      ("some / path", "some . path"),
      (r#"C:\example\folder"#, ".C.example.folder"),
      ("/example/folder", ".example.folder"),
      ("test/text.txt", "test.text"),
      ("test/script.lua", "test.script"),
      ("test/lib.dll", "test.lib"),
    ];

    for (p, r) in tests {
      assert_eq!(r.to_string(), path::into_id(Path::new(p)));
    }
  }
}
