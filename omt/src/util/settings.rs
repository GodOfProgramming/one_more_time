use toml::Value;

pub trait Settings {
  fn modify(&mut self, name: &str, f: Box<dyn FnOnce(&mut Value)>);
}
