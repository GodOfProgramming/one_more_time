use super::UiRoot;
use crate::util::XmlNode;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct UiManager {
  elements: BTreeMap<String, UiRoot>,
}

impl UiManager {
  fn new<I>(iter: I) -> Self
  where
    I: Iterator,
    I::Item: ToString,
  {
    let mut manager = Self::default();

    // ? why is storing entry.to_string() not allowed
    for entry in iter {
      if let Ok(xml) = std::fs::read_to_string(entry.to_string()) {
        if let Ok(nodes) = XmlNode::parse(&xml) {
          for node in nodes {
            manager
              .elements
              .insert(entry.to_string(), UiRoot::from(node));
          }
        }
      }
    }

    manager
  }
}
