use super::{UiComponentPtr, UiTemplate};
use crate::{
  scripting::{LuaType, LuaTypeTrait, ScriptRepository},
  util::{DirID, Logger, Settings, XmlNode},
};
use imgui_glium_renderer::imgui::Ui;
use mlua::{UserData, UserDataMethods, Value};
use std::{
  collections::{BTreeMap, HashMap},
  path::PathBuf,
};

static mut NEXT_ID: usize = 0;

#[derive(Default)]
pub struct UiManager {
  templates: BTreeMap<DirID, UiTemplate>,
  open_ui: BTreeMap<String, UiComponentPtr>,
}

impl UiManager {
  pub fn new<L: Logger, I>(logger: &L, iter: I, scripts: &ScriptRepository) -> Self
  where
    I: Iterator<Item = (PathBuf, DirID)>,
  {
    let mut manager = Self::default();

    logger.debug("reading entries".to_string());

    for (entry, id) in iter {
      logger.debug(format!("reading entry {:?}", entry));
      if let Ok(xml) = std::fs::read_to_string(&entry) {
        if let Ok(mut nodes) = XmlNode::parse(&xml) {
          if let Some(node) = nodes.drain(..).next() {
            println!("storing {:?}", id);
            manager
              .templates
              .insert(id, UiTemplate::new(node, logger, scripts));
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

  #[profiling::function]
  pub fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    for element in self.open_ui.values_mut() {
      element.component().update(ui, settings);
    }
  }

  pub fn open(&mut self, id: &str, name: &str, data: Value) -> Option<UiComponentPtr> {
    if let Some(tmpl) = self.templates.get(&DirID::from(id)) {
      let component = tmpl.create_component(data);
      self.open_ui.insert(name.to_string(), component.clone());
      Some(component)
    } else {
      None
    }
  }

  pub fn get(&self, name: &str) -> Option<UiComponentPtr> {
    self.open_ui.get(name).cloned()
  }
}

impl LuaTypeTrait for UiManager {}

impl LuaType<UiManager> {
  fn open(&mut self, id: &str, name: String, data: Value) -> Option<UiComponentPtr> {
    self.obj_mut().open(id, &name, data)
  }

  fn get(&self, name: &str) -> Option<UiComponentPtr> {
    self.obj().get(name)
  }
}

impl UserData for LuaType<UiManager> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut(
      "open",
      |_, this, (id, name, data): (String, String, Value)| {
        if let Some(mut ptr) = this.open(&id, name, data) {
          Ok(Some(ptr.component().create_lua_type()))
        } else {
          Ok(None)
        }
      },
    );

    methods.add_method_mut("get", |_, this, name: String| {
      if let Some(mut ptr) = this.get(&name) {
        Ok(Some(ptr.component().create_lua_type()))
      } else {
        Ok(None)
      }
    });
  }
}
