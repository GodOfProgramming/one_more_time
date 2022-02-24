use toml::Value;

pub trait Settings {
  fn modify(&mut self, name: &str, f: &dyn FnOnce(&mut Value));
}
