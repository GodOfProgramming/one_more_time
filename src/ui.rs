pub mod main_menu_bar;
pub mod menu;
pub mod menu_item;
pub mod text_box;
pub mod ui_manager;
pub mod window;

pub mod components {
  pub use super::main_menu_bar::*;
  pub use super::menu::*;
  pub use super::menu_item::*;
  pub use super::text_box::*;
  pub use super::window::*;
}

use crate::{
  scripting::{LuaType, LuaTypeTrait, ScriptRepository},
  util::{Logger, Settings, XmlNode},
};
use dyn_clone::DynClone;
use imgui_glium_renderer::imgui::Ui;
use lazy_static::lazy_static;
use log::warn;
use maplit::hashmap;
use mlua::{prelude::*, UserData, UserDataMethods, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub use ui_manager::UiManager;

#[macro_export]
macro_rules! type_map {
    [ $( $feat:ident ),* ] => {
      hashmap! {
        $(
          $feat.0 => $feat.1,
        )*
      }
    };
  }

pub mod common {
  pub use super::{types, SubElementMap, UiElement, UiElementParent, UiSubElements};
  pub use crate::{
    type_map,
    util::{convert::string, ChildLogger, Settings, XmlNode},
  };
  pub use imgui_glium_renderer::imgui::{self, ImStr, Ui};
  pub use lazy_static::lazy_static;
  pub use maplit::hashmap;
  pub use mlua::prelude::*;
  pub use std::ffi::CString;
}

pub mod types {
  use super::{components::*, SubElementCreator, UiSubElement, XmlNode};

  type UiComponentKvp = (&'static str, SubElementCreator);

  fn create_window(root: XmlNode) -> UiSubElement {
    Box::new(Window::new(root))
  }

  fn create_textbox(root: XmlNode) -> UiSubElement {
    Box::new(TextBox::new(root))
  }

  fn create_main_menu_bar(root: XmlNode) -> UiSubElement {
    Box::new(MainMenuBar::new(root))
  }

  fn create_menu(root: XmlNode) -> UiSubElement {
    Box::new(Menu::new(root))
  }

  fn create_menu_item(root: XmlNode) -> UiSubElement {
    Box::new(MenuItem::new(root))
  }

  pub const WINDOW: UiComponentKvp = ("window", create_window);
  pub const TEXTBOX: UiComponentKvp = ("textbox", create_textbox);
  pub const MAIN_MENU_BAR: UiComponentKvp = ("main-menu-bar", create_main_menu_bar);
  pub const MENU: UiComponentKvp = ("menu", create_menu);
  pub const MENU_ITEM: UiComponentKvp = ("menu-item", create_menu_item);
}

pub type UiSubElement = Box<dyn UiElement + Send + Sync>;
pub type UiSubElements = Vec<UiSubElement>;
pub type SubElementCreator = fn(XmlNode) -> UiSubElement;
pub type SubElementMap = HashMap<&'static str, SubElementCreator>;

pub trait UiElement: DynClone {
  fn update(&mut self, _ui: &Ui<'_>, _lua: Option<&Lua>, _settings: &Settings) {
    panic!("'update' unimplemented");
  }
}

dyn_clone::clone_trait_object!(UiElement);

pub trait UiElementParent {
  fn valid_children() -> &'static SubElementMap;
}

#[derive(Clone)]
struct EmptyUi;

impl UiElement for EmptyUi {}

pub fn parse_children<E: UiElementParent>(root: XmlNode) -> UiSubElements {
  let parse_child = |root: XmlNode| -> Option<UiSubElement> {
    if let Some(f) = E::valid_children().get(root.name.as_str()) {
      Some(f(root))
    } else {
      warn!("ui type of '{}' is not valid", root.name);
      None
    }
  };

  let mut children = Vec::new();

  for child in root.children {
    if let Some(child) = parse_child(child) {
      children.push(child);
    }
  }

  children
}

pub struct UiTemplate {
  el: UiSubElement,
  lua: Option<Rc<RefCell<Lua>>>,
}

impl UiTemplate {
  pub fn new<L: Logger>(node: XmlNode, logger: &L, scripts: &ScriptRepository) -> Self {
    let mut root = UiTemplate::default();

    for node in node.children {
      if node.name == "script" {
        if let Some(id) = node.attribs.get("id") {
          if let Some(lua) = scripts.get(id) {
            root.lua = Some(lua);
          }
        } else {
          logger.error("script tag without id found".to_string());
        }
      } else if let Some(f) = Self::valid_children().get(node.name.as_str()) {
        root.el = f(node);
      } else {
        logger.error(format!("ui type of '{}' is not valid", node.name));
      }
    }

    root
  }

  pub fn create_component(&self, data: Value) -> UiComponent {
    let mut component = UiComponent::new(self.lua.clone());

    component.el = self.el.clone();

    component.initialize(data);

    component
  }
}

impl Default for UiTemplate {
  fn default() -> Self {
    UiTemplate {
      el: Box::new(EmptyUi),
      lua: None,
    }
  }
}

impl UiElementParent for UiTemplate {
  fn valid_children() -> &'static SubElementMap {
    use types::{MAIN_MENU_BAR, WINDOW};

    lazy_static! {
      static ref MAP: SubElementMap = type_map![WINDOW, MAIN_MENU_BAR];
    }

    &MAP
  }
}

pub struct UiComponent {
  el: UiSubElement,
  lua: Option<Rc<RefCell<Lua>>>,
}

impl UiComponent {
  fn new(lua: Option<Rc<RefCell<Lua>>>) -> Self {
    Self {
      lua,
      el: Box::new(EmptyUi),
    }
  }

  fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    if let Some(lua) = &self.lua {
      self.el.update(ui, Some(&lua.borrow()), settings);
    } else {
      self.el.update(ui, None, settings);
    }
  }

  fn initialize(&self, data: Value) {
    if let Some(lua) = &self.lua {
      let lua = lua.borrow();
      let globals = lua.globals();
      if let Ok(true) = globals.contains_key("initialize") {
        let res: Result<(), mlua::Error> = globals.call_function("initialize", data);
        if let Err(e) = res {
          println!("error {}", e);
        }
      }
    }
  }

  fn get_element_by_id(&self, id: String) -> Option<LuaType<UiComponent>> {
    None // TODO
  }
}

impl LuaTypeTrait for UiComponent {}

impl LuaType<UiComponent> {
  fn get_element_by_id(&self, id: String) -> Option<LuaType<UiComponent>> {
    self.obj().get_element_by_id(id)
  }
}

impl UserData for LuaType<UiComponent> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("get_element_by_id", |_, this, id: String| {
      Ok(this.get_element_by_id(id))
    });
  }
}
