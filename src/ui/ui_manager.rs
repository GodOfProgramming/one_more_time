use super::UiRoot;
use crate::{
  ui::UiElement,
  util::{DirID, Settings, XmlNode},
};
use imgui_glium_renderer::imgui::Ui;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
pub struct UiManager {
  ui: BTreeMap<DirID, UiRoot>,
}

impl UiManager {
  pub fn new<I>(iter: I) -> Self
  where
    I: Iterator<Item = PathBuf>,
  {
    let mut manager = Self::default();

    for entry in iter {
      if let Ok(xml) = std::fs::read_to_string(&entry) {
        if let Ok(mut nodes) = XmlNode::parse(&xml) {
          if let Some(node) = nodes.drain(..).next() {
            manager.ui.insert(DirID::from(entry), UiRoot::from(node));
          } else {
            // todo log here
          }
        }
      }
    }

    manager
  }

  pub fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    for element in self.ui.values_mut() {
      element.update(ui, settings);
    }
  }
}
