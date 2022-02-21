use super::{common::*, UiComponent, UiComponentPtr, UiTemplate};
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

  pub fn open(&mut self, id: &str, name: &str) -> Option<MutPtr<UiComponent>> {
    if let Some(tmpl) = self.templates.get(&DirID::from(id)) {
      let mut component = tmpl.create_component(&self.logger);
      let ptr = component.as_ptr_mut();
      self.open_ui.insert(name.to_string(), component);
      Some(ptr)
    } else {
      None
    }
  }

  pub fn get(&mut self, name: &str) -> Option<MutPtr<UiComponent>> {
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

impl UserData for MutPtr<UiManager> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("open", |_, this, (id, name): (String, String)| {
      if let Some(ptr) = this.open(&id, &name) {
        Ok(Some(ptr))
      } else {
        Ok(None)
      }
    });

    methods.add_method_mut("get", |_, this, name: String| {
      if let Some(ptr) = this.get(&name) {
        Ok(Some(ptr))
      } else {
        Ok(None)
      }
    });

    methods.add_method("list", |_, this, _: ()| Ok(this.list()))
  }
}
