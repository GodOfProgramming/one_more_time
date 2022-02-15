use super::{UiComponent, UiTemplate};
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
  open_ui: BTreeMap<UiHandle, UiComponent>,
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

  pub fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    for element in self.open_ui.values_mut() {
      element.update(ui, settings);
    }
  }

  pub fn open(&mut self, id: &str, data: Value) {
    if let Some(tmpl) = self.templates.get(&DirID::from(id)) {
      println!("opening {}", id);
      self
        .open_ui
        .insert(UiHandle::next(), tmpl.create_component(data));
      println!("opened {}", id);
    }
  }
}

impl LuaTypeTrait for UiManager {}

impl LuaType<UiManager> {
  fn open(&mut self, id: &str, data: Value) {
    self.obj_mut().open(id, data);
  }
}

impl UserData for LuaType<UiManager> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("open", |_, this, (id, data): (String, Value)| {
      this.open(&id, data);
      Ok(())
    });
  }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct UiHandle {
  id: usize,
}

impl UiHandle {
  fn next() -> Self {
    let id: usize;
    unsafe {
      id = NEXT_ID;
      NEXT_ID += 1;
    };
    Self { id }
  }
}
