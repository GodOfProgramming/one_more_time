use super::UiRoot;
use crate::{
  ui::UiElement,
  util::{DirID, Logger, Settings, XmlNode},
};
use imgui_glium_renderer::imgui::Ui;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
pub struct UiManager {
  ui: BTreeMap<DirID, UiRoot>,
}

impl UiManager {
  pub fn new<L: Logger, I>(logger: &L, iter: I) -> Self
  where
    I: Iterator<Item = PathBuf>,
  {
    let mut manager = Self::default();

    logger.debug("reading entries".to_string());

    for entry in iter {
      logger.debug(format!("reading entry {:?}", entry));
      if let Ok(xml) = std::fs::read_to_string(&entry) {
        if let Ok(mut nodes) = XmlNode::parse(&xml) {
          if let Some(node) = nodes.drain(..).next() {
            manager
              .ui
              .insert(DirID::from(entry), UiRoot::from((logger, node)));
          } else {
            logger.error(format!("xml {:?} was empty", entry));
          }
        } else {
          logger.error(format!("failed to parse xml for {:?}", entry));
        }
      } else {
        logger.error(format!("unable to read file {:?}", entry));
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
