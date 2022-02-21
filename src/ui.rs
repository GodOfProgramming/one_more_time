use common::*;
use dyn_clone::DynClone;
use imgui_glium_renderer::imgui;
use lazy_static::lazy_static;
use log::warn;
use maplit::hashmap;
use mlua::{Scope, Value};
use std::{
  collections::{BTreeMap, HashMap},
  ops::{Deref, DerefMut},
};

pub use ui_manager::UiManager;
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

pub mod common {
  pub use super::{types, SubElementMap, Ui, UiElement, UiElementParent, UiElementPtr};
  pub use crate::{type_map, util::prelude::*};
  pub use imgui_glium_renderer::imgui::{self, ImStr};
  pub use lazy_static::lazy_static;
  pub use maplit::hashmap;
  pub use mlua::{prelude::*, UserData, UserDataFields, UserDataMetatable, UserDataMethods, Value};
  pub use std::{collections::BTreeMap, ffi::CString, rc::Rc};
}

pub mod types {
  use super::{components::*, SubElementCreator, Ui, XmlNode};

  type UiComponentKvp = (&'static str, SubElementCreator);

  fn create_window(root: XmlNode) -> Ui {
    Window::new(root).into()
  }

  fn create_textbox(root: XmlNode) -> Ui {
    TextBox::new(root).into()
  }

  fn create_main_menu_bar(root: XmlNode) -> Ui {
    MainMenuBar::new(root).into()
  }

  fn create_menu(root: XmlNode) -> Ui {
    Menu::new(root).into()
  }

  fn create_menu_item(root: XmlNode) -> Ui {
    MenuItem::new(root).into()
  }

  pub const WINDOW: UiComponentKvp = ("window", create_window);
  pub const TEXTBOX: UiComponentKvp = ("textbox", create_textbox);
  pub const MAIN_MENU_BAR: UiComponentKvp = ("main-menu-bar", create_main_menu_bar);
  pub const MENU: UiComponentKvp = ("menu", create_menu);
  pub const MENU_ITEM: UiComponentKvp = ("menu-item", create_menu_item);
}

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

pub type SubElementCreator = fn(XmlNode) -> Ui;
pub type SubElementMap = HashMap<&'static str, SubElementCreator>;
pub type UiElementPtr = Box<dyn UiElement>;

pub trait UiElement: DynClone {
  fn kind(&self) -> String;

  fn id(&self) -> Option<String>;

  fn set_attrib(&mut self, _attrib: String, _value: Value) {
    panic!("'set_attrib' not implemented")
  }

  fn update(
    &mut self,
    _logger: &dyn Logger,
    _ui: &imgui::Ui<'_>,
    _class: &LuaValue,
    _instance: &LuaValue,
    _settings: &Settings,
  ) {
    panic!("'update' not implemented");
  }

  fn dupe(&self) -> UiElementPtr;

  fn clone_ui(&self, id_map: &mut BTreeMap<String, Ui>) -> Ui {
    let ui = Ui(self.dupe());

    if let Some(id) = self.id() {
      id_map.insert(id, ui.clone());
    }

    ui
  }
}

dyn_clone::clone_trait_object!(UiElement);

pub trait UiElementParent {
  fn valid_children() -> &'static SubElementMap;
}

#[derive(Clone)]
pub struct Ui(UiElementPtr);

impl Ui {
  fn el(&self) -> &dyn UiElement {
    self.0.as_ref()
  }

  fn el_mut(&mut self) -> &mut dyn UiElement {
    self.0.as_mut()
  }
}

impl UiElement for Ui {
  fn kind(&self) -> String {
    self.el().kind()
  }

  fn id(&self) -> Option<String> {
    self.el().id()
  }

  fn set_attrib(&mut self, attrib: String, value: Value) {
    self.el_mut().set_attrib(attrib, value);
  }

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    class: &LuaValue,
    instance: &LuaValue,
    settings: &Settings,
  ) {
    self.el_mut().update(logger, ui, class, instance, settings);
  }

  fn dupe(&self) -> UiElementPtr {
    self.el().dupe()
  }
}

impl AsPtr for Ui {}

impl UserData for MutPtr<Ui> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("kind", |_, this, _: ()| Ok(this.kind()));
    methods.add_method_mut("set_attrib", |_, this, (name, value): (String, Value)| {
      this.set_attrib(name, value);
      Ok(())
    });
  }
}

pub fn parse_children<E: UiElementParent>(root: XmlNode) -> Vec<Ui> {
  let parse_child = |root: XmlNode| -> Option<Ui> {
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
  el: Ui,
  class: LuaValue<'static>,
}

impl Default for UiTemplate {
  fn default() -> Self {
    Self {
      el: EmptyUi::default().into(),
      class: LuaValue::Nil,
    }
  }
}

impl UiTemplate {
  pub fn new<L: Logger>(node: XmlNode, logger: &L, lua: &'static Lua) -> Self {
    let mut root = UiTemplate::default();

    for node in node.children {
      if node.name == "model" {
        if let Some(class_name) = node.attribs.get("class") {
          match script::resolve(lua, class_name) {
            Ok(class) => root.class = class,
            Err(err) => logger.error(format!("cannot find class '{}': {}", class_name, err)),
          }
        } else {
          logger.error("model tag without class name found".to_string());
        }
      } else if let Some(f) = Self::valid_children().get(node.name.as_str()) {
        root.el = f(node);
      } else {
        logger.error(format!("ui type of '{}' is not valid", node.name));
      }
    }

    root
  }

  pub fn create_component<L: Logger>(&self, logger: &L) -> UiComponentPtr {
    let mut id_map = BTreeMap::new();
    let el = self.el.clone_ui(&mut id_map);

    let (class, instance) = if let LuaValue::Table(class) = &self.class {
      match class.call_function("new", class.clone()) {
        Ok(instance) => (LuaValue::Table(class.clone()), LuaValue::Table(instance)),
        Err(err) => {
          logger.error(format!("constructor failed: {}", err));
          (LuaValue::Table(class.clone()), LuaValue::Nil)
        }
      }
    } else {
      (LuaValue::Nil, LuaValue::Nil)
    };

    let component = UiComponent::new(el, class, instance, id_map);

    UiComponentPtr::new(component)
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
  el: Ui,
  class: LuaValue<'static>,
  instance: LuaValue<'static>,
  element_mapping: BTreeMap<String, Ui>,
}

impl UiComponent {
  fn new(
    el: Ui,
    class: LuaValue<'static>,
    instance: LuaValue<'static>,
    element_mapping: BTreeMap<String, Ui>,
  ) -> Self {
    Self {
      el,
      class,
      instance,
      element_mapping,
    }
  }

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    lua: &'static Lua,
    settings: &Settings,
  ) {
    let ptr = self.as_ptr_mut();
    let res: mlua::Result<()> = lua.scope(|scope: &mlua::Scope| {
      let ptr = scope.create_userdata(ptr).unwrap();
      let globals = lua.globals();
      let _ = globals.set("document", ptr);

      self
        .el
        .update(logger, ui, &self.class, &self.instance, settings);

      globals.set("document", LuaValue::Nil).unwrap();
      Ok(())
    });

    if let Err(e) = res {
      logger.error(e.to_string());
    }
  }

  fn get_element_by_id(&mut self, id: &str) -> Option<MutPtr<Ui>> {
    self.element_mapping.get_mut(id).map(|ui| ui.as_ptr_mut())
  }
}

impl AsPtr for UiComponent {}

impl UserData for MutPtr<UiComponent> {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("data", |_, this| Ok(this.instance.clone()));
  }

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("get_element_by_id", |_, this, id: String| {
      Ok(this.get_element_by_id(&id))
    });
  }
}

pub struct UiComponentPtr(Box<UiComponent>);

impl UiComponentPtr {
  pub fn new(ui: UiComponent) -> Self {
    Self(Box::new(ui))
  }

  pub fn component(&mut self) -> &mut UiComponent {
    self.0.as_mut()
  }
}

impl Deref for UiComponentPtr {
  type Target = UiComponent;
  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

impl DerefMut for UiComponentPtr {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.as_mut()
  }
}

#[derive(Default, Clone)]
struct EmptyUi;

impl UiElement for EmptyUi {
  fn kind(&self) -> String {
    String::from("EmptyUi")
  }

  fn id(&self) -> Option<String> {
    None
  }

  fn dupe(&self) -> UiElementPtr {
    Box::new(EmptyUi)
  }
}

impl From<EmptyUi> for Ui {
  fn from(ui: EmptyUi) -> Self {
    Ui(Box::new(ui))
  }
}
