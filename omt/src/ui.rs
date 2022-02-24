use std::{
  cell::RefCell,
  fmt::{Display, Error, Formatter},
  rc::Rc,
};

pub enum UiAttributeValue {
  Int(i64),
  Uint(u64),
  String(String),
  Bool(bool),
}

#[derive(Debug)]
pub enum UiModelError {
  NoError,               // used in instances where a result was desired
  StandardError(String), // simple message string
}

impl Display for UiModelError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      UiModelError::NoError => Ok(()),
      UiModelError::StandardError(msg) => write!(f, "{}", msg),
    }
  }
}

pub trait UiModel {
  fn tag_name(&self) -> &'static str;
  fn new_instance(&self) -> Result<Box<dyn UiModelInstance>, UiModelError>;
}

pub trait UiModelInstance {
  fn call_handler(&self, _name: &str) {}
}

pub trait Document {
  fn get_element_by_id(&mut self, id: &str) -> Option<Rc<RefCell<dyn UiElement>>>;
}

pub trait UiElement {
  fn kind(&self) -> String;
  fn id(&self) -> Option<String>;
  fn set_attrib(&mut self, attrib: String, value: UiAttributeValue);
}

// Built-in types

pub struct StaticUi;

impl UiModel for StaticUi {
  fn tag_name(&self) -> &'static str {
    "static"
  }

  fn new_instance(&self) -> Result<Box<dyn UiModelInstance>, UiModelError> {
    Ok(Box::new(StaticUi))
  }
}

impl UiModelInstance for StaticUi {}
