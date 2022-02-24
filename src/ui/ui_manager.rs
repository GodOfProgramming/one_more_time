use super::{common::*, UiComponentInstance, UiPtr, UiTemplate};
use crate::util::prelude::*;
use omt::{imgui_glium_renderer::imgui::Ui, profiling, ui::UiModel};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default)]
pub struct UiModelArchive {
  models: BTreeMap<String, Rc<dyn UiModel>>,
}

impl UiModelArchive {
  pub fn lookup(&self, name: &str) -> Option<Rc<dyn UiModel>> {
    self.models.get(name).cloned()
  }
}

pub struct UiManager {
  logger: ChildLogger,
  archive: UiModelArchive,
  templates: BTreeMap<DirID, UiTemplate>,
  open_ui: BTreeMap<String, UiPtr>,
}

impl UiManager {
  pub fn new(logger: ChildLogger) -> Self {
    Self {
      logger,
      archive: Default::default(),
      templates: Default::default(),
      open_ui: Default::default(),
    }
  }

  pub fn load_ui<I>(&mut self, iter: I)
  where
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    self.logger.debug("loading ui".to_string());

    for (entry, id) in iter {
      self.logger.debug(format!("reading entry {:?}", entry));
      if let Ok(xml) = std::fs::read_to_string(&entry) {
        if let Ok(mut nodes) = XmlNode::parse(&xml) {
          if let Some(node) = nodes.drain(..).next() {
            self
              .templates
              .insert(id, UiTemplate::new(node, &self.archive, &self.logger));
          } else {
            self.logger.error(format!("xml {:?} was empty", entry));
          }
        } else {
          self
            .logger
            .error(format!("failed to parse xml for {:?}", entry));
        }
      } else {
        self
          .logger
          .error(format!("unable to read file {:?}", entry));
      }
    }
  }

  #[profiling::function]
  pub fn update(&mut self, logger: &dyn Logger, ui: &Ui<'_>, settings: &Settings) {
    for element in self.open_ui.values_mut() {
      element.update(logger, ui, settings);
    }
  }

  pub fn open(&mut self, id: &str, name: &str) -> Option<MutPtr<UiComponentInstance>> {
    if let Some(tmpl) = self.templates.get(&DirID::from(id)) {
      let mut component = tmpl.create_component(&self.logger);
      let ptr = component.as_ptr_mut();
      self.open_ui.insert(name.to_string(), component);
      Some(ptr)
    } else {
      None
    }
  }

  pub fn get(&mut self, name: &str) -> Option<MutPtr<UiComponentInstance>> {
    self.open_ui.get_mut(name).map(|c| c.as_ptr_mut())
  }

  pub fn list(&self) -> Vec<String> {
    let mut ret = Vec::default();

    for id in self.open_ui.keys() {
      ret.push(id.clone());
    }

    ret
  }
}

impl AsPtr for UiManager {}
