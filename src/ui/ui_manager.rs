use super::{common::*, StaticUi, UiComponentInstance, UiPtr, UiTemplate};
use crate::{imgui::Ui, util::prelude::*};
use omt::ui::{UiModel, UiModelLoader, UiSourceLoader};
use profiling;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct UiModelArchive {
  models: BTreeMap<String, Rc<dyn UiModel>>,
}

impl UiModelLoader for UiModelArchive {
  fn register(&mut self, name: &str, model: Rc<dyn UiModel>) {
    self.models.insert(name.to_string(), model);
  }
}

#[derive(Default)]
pub struct UiTemplateSourceArchive {
  xml: BTreeMap<String, String>,
}

impl UiSourceLoader for UiTemplateSourceArchive {
  fn register(&mut self, name: String, xml: String) {
    self.xml.insert(name, xml);
  }
}

pub struct UiManager {
  logger: ChildLogger,
  models: BTreeMap<String, Rc<dyn UiModel>>,
  templates: BTreeMap<String, UiTemplate>,
  open_ui: BTreeMap<String, UiPtr>,
}

impl UiManager {
  pub fn new(logger: ChildLogger) -> Self {
    let mut models: BTreeMap<String, Rc<dyn UiModel>> = BTreeMap::default();

    models.insert("static".to_string(), Rc::new(StaticUi));

    Self {
      logger,
      models,
      templates: Default::default(),
      open_ui: Default::default(),
    }
  }

  pub fn add_model_archive(&mut self, archive: UiModelArchive) {
    for (id, model) in archive.models {
      self.models.insert(id, model);
    }
  }

  pub fn add_template_archive(&mut self, archive: UiTemplateSourceArchive) {
    for (id, source) in archive.xml {
      if let Ok(mut nodes) = XmlNode::parse(&source) {
        if let Some(node) = nodes.drain(..).next() {
          self
            .templates
            .insert(id, UiTemplate::new(node, &self.models, &self.logger));
        } else {
          self.logger.error(format!("xml {:?} was empty", id));
        }
      } else {
        self
          .logger
          .error(format!("failed to parse xml for {:?}", id));
      }
    }
  }

  #[profiling::function]
  pub fn update(&mut self, ui: &Ui<'_>, game: &mut dyn Game) {
    for element in self.open_ui.values_mut() {
      element.update(ui, game);
    }
  }

  pub fn open(&mut self, id: &str, name: &str) -> Option<MutPtr<UiComponentInstance>> {
    if let Some(tmpl) = self.templates.get(id) {
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
