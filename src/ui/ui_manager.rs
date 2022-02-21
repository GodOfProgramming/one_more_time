use super::{common::*, UiComponentPtr, UiTemplate};
use crate::util::prelude::*;
use imgui_glium_renderer::imgui::Ui;
use mlua::{Lua, UserData, UserDataMethods, Value};
use std::{collections::BTreeMap, path::PathBuf};

pub struct UiManager {
  templates: BTreeMap<DirID, UiTemplate>,
  open_ui: BTreeMap<String, UiComponentPtr>,
  logger: ChildLogger,
}

impl UiManager {
  pub fn new(logger: ChildLogger) -> Self {
    Self {
      logger,
      templates: Default::default(),
      open_ui: Default::default(),
    }
  }

  pub fn load_ui<I>(&mut self, iter: I, lua: &'static Lua)
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
              .insert(id, UiTemplate::new(node, &self.logger, lua));
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
  pub fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &Ui<'_>,
    lua: &'static Lua,
    settings: &Settings,
  ) {
    for element in self.open_ui.values_mut() {
      element.component().update(logger, ui, lua, settings);
    }
  }

  pub fn open(&mut self, id: &str, name: &str) -> Option<UiComponentPtr> {
    if let Some(tmpl) = self.templates.get(&DirID::from(id)) {
      let component = tmpl.create_component(&self.logger);
      self.open_ui.insert(name.to_string(), component.clone());
      Some(component)
    } else {
      None
    }
  }

  pub fn get(&self, name: &str) -> Option<UiComponentPtr> {
    self.open_ui.get(name).cloned()
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

impl UserData for MutPtr<UiManager> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("open", |_, this, (id, name): (String, String)| {
      if let Some(mut ptr) = this.open(&id, &name) {
        Ok(Some(ptr.component().as_ptr_mut()))
      } else {
        Ok(None)
      }
    });

    methods.add_method("get", |_, this, name: String| {
      if let Some(mut ptr) = this.get(&name) {
        Ok(Some(ptr.component().as_ptr_mut()))
      } else {
        Ok(None)
      }
    });

    methods.add_method("list", |_, this, _: ()| Ok(this.list()))
  }
}
