use toml::Value;

pub trait Settings {
  fn lookup(&self, path: &[&str]) -> &Value;
  fn modify(&mut self, path: &[&str], f: Box<dyn FnOnce(&mut Value)>);
}
