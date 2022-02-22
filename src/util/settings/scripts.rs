use super::common::*;
use omt::{regex::Regex, toml::Value};
use std::str::FromStr;

mod keys {
  pub const EXCLUDE: &str = "exclude";
}

#[derive(Debug, Default)]
pub struct Settings {
  pub exclude: Vec<Regex>,
}

impl From<&Table> for Settings {
  fn from(table: &Table) -> Self {
    let mut settings = Self::default();

    if let Some(Value::Array(excludes)) = table.get(keys::EXCLUDE) {
      for exclude in excludes {
        if let Value::String(exclude) = exclude {
          settings.exclude.push(Regex::from_str(exclude).unwrap());
        }
      }
    }

    settings
  }
}

impl From<Table> for Settings {
  fn from(table: Table) -> Self {
    Self::from(&table)
  }
}

impl Into<Table> for Settings {
  fn into(self) -> Table {
    let mut table = Table::new();

    table.insert(
      String::from(keys::EXCLUDE),
      Value::Array(
        self
          .exclude
          .into_iter()
          .map(|s| Value::String(s.to_string()))
          .collect(),
      ),
    );

    table
  }
}

impl AsPtr for Settings {}
