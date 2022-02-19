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
  pub use crate::{
    type_map,
    util::{convert::string, ChildLogger, Settings, XmlNode},
  };
  pub use imgui_glium_renderer::imgui::{self, ImStr};
  pub use lazy_static::lazy_static;
  pub use maplit::hashmap;
  pub use mlua::{prelude::*, Value};
  pub use std::{cell::RefCell, collections::BTreeMap, ffi::CString, rc::Rc};
}

use crate::{
  scripting::{LuaType, LuaTypeTrait, ScriptRepository},
  util::{Logger, Settings, XmlNode},
};

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

use dyn_clone::DynClone;
use imgui_glium_renderer::imgui;
use lazy_static::lazy_static;
use log::warn;
use maplit::hashmap;
use mlua::{prelude::*, UserData, UserDataMethods, Value};
use std::{
  cell::RefCell,
  collections::{BTreeMap, HashMap},
  rc::Rc,
};
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

pub type SubElementCreator = fn(XmlNode) -> Ui;
pub type SubElementMap = HashMap<&'static str, SubElementCreator>;
pub type UiElementPtr = Rc<RefCell<dyn UiElement>>;

pub trait UiElement: DynClone {
  fn kind(&self) -> String;

  fn id(&self) -> Option<String>;

  fn set_attrib(&mut self, _attrib: String, _value: Value) {
    panic!("'set_attrib' not implemented")
  }

  fn update(&mut self, _ui: &imgui::Ui<'_>, _lua: Option<&Lua>, _settings: &Settings) {
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
    unsafe { &*self.0.as_ptr() }
  }

  fn el_mut(&mut self) -> &mut dyn UiElement {
    unsafe { &mut *self.0.as_ptr() }
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

  fn update(&mut self, ui: &imgui::Ui<'_>, lua: Option<&Lua>, settings: &Settings) {
    self.el_mut().update(ui, lua, settings);
  }

  fn dupe(&self) -> UiElementPtr {
    self.el().dupe()
  }
}

impl LuaType<Ui> {}

impl UiElement for LuaType<Ui> {
  fn kind(&self) -> String {
    self.obj().kind()
  }

  fn id(&self) -> Option<String> {
    self.obj().id()
  }

  fn set_attrib(&mut self, attrib: String, value: Value) {
    self.obj_mut().set_attrib(attrib, value);
  }

  fn dupe(&self) -> UiElementPtr {
    self.obj().dupe()
  }
}

impl LuaTypeTrait for Ui {}

impl UserData for LuaType<Ui> {
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
  lua: Option<Rc<Lua>>,
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

  pub fn create_component(&self, data: Value) -> UiComponentPtr {
    let mut id_map = BTreeMap::new();
    let el = self.el.clone_ui(&mut id_map);
    let component = UiComponent::new(self.lua.clone(), el, id_map);

    component.initialize(data);

    UiComponentPtr::new(component)
  }
}

impl Default for UiTemplate {
  fn default() -> Self {
    UiTemplate {
      el: EmptyUi::default().into(),
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
  el: Ui,
  lua: Option<Rc<Lua>>,
  element_mapping: BTreeMap<String, Ui>,
}

impl UiComponent {
  fn new(lua: Option<Rc<Lua>>, el: Ui, element_mapping: BTreeMap<String, Ui>) -> Self {
    Self {
      lua,
      el,
      element_mapping,
    }
  }

  fn update(&mut self, ui: &imgui::Ui<'_>, settings: &Settings) {
    let lua_type = self.create_lua_type();
    if let Some(lua) = &self.lua {
      unsafe {
        let _ = lua.globals().set("document", lua_type);
        self.el.update(ui, Some(&lua), settings);
      }
    } else {
      self.el.update(ui, None, settings);
    }
  }

  fn initialize(&self, data: Value) {
    if let Some(lua) = &self.lua {
      let globals = lua.globals();
      if let Ok(true) = globals.contains_key("initialize") {
        let res: Result<(), mlua::Error> = globals.call_function("initialize", data);
        if let Err(_e) = res {
          // todo
        }
      }
    }
  }

  fn get_element_by_id(&mut self, id: String) -> Option<LuaType<Ui>> {
    self
      .element_mapping
      .get_mut(&id)
      .map(|ui| ui.create_lua_type())
  }
}

impl LuaType<UiComponent> {
  fn get_element_by_id(&mut self, id: String) -> Option<LuaType<Ui>> {
    self.obj_mut().get_element_by_id(id)
  }
}

impl LuaTypeTrait for UiComponent {}

impl UserData for LuaType<UiComponent> {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("get_element_by_id", |_, this, id: String| {
      Ok(this.get_element_by_id(id))
    });
  }
}

#[derive(Clone)]
pub struct UiComponentPtr(Rc<RefCell<UiComponent>>);

impl UiComponentPtr {
  pub fn new(ui: UiComponent) -> Self {
    Self(Rc::new(RefCell::new(ui)))
  }

  pub fn component(&mut self) -> &mut UiComponent {
    unsafe { &mut *self.0.as_ptr() }
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
    Rc::new(RefCell::new(EmptyUi))
  }
}

impl From<EmptyUi> for Ui {
  fn from(ui: EmptyUi) -> Self {
    Ui(Rc::new(RefCell::new(ui)))
  }
}
