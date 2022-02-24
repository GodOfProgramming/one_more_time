use common::*;
pub use omt::{
  core::Game,
  dyn_clone::{self, DynClone},
  imgui,
  ui::{Document, StaticUi, UiElement, UiModel, UiModelInstance},
};
use std::collections::{BTreeMap, HashMap};

pub use ui_manager::{UiManager, UiModelArchive};
pub mod main_menu_bar;
pub mod menu;
pub mod menu_item;
pub mod text_box;
pub mod ui_manager;
pub mod window;

/* ----------------------------- Utility ----------------------------- */

pub mod components {
  pub use super::main_menu_bar::*;
  pub use super::menu::*;
  pub use super::menu_item::*;
  pub use super::text_box::*;
  pub use super::window::*;
}

pub mod common {
  pub use super::{
    types, SubElementMap, UiComponent, UiComponentPtr, UiElementParent, UiElementPtr,
  };
  pub use crate::{type_map, util::prelude::*};
  pub use omt::{
    core::Game,
    imgui::{self, ImStr},
    lazy_static::lazy_static,
    maplit::hashmap,
    ui::{UiAttributeValue, UiElement, UiModelInstance},
  };
  pub use std::{cell::RefCell, collections::BTreeMap, ffi::CString, rc::Rc};
}

pub mod types {
  use super::{components::*, SubElementCreator, UiComponentPtr, XmlNode};

  type UiComponentKvp = (&'static str, SubElementCreator);

  fn create_window(root: XmlNode) -> UiComponentPtr {
    Window::new(root).into()
  }

  fn create_textbox(root: XmlNode) -> UiComponentPtr {
    TextBox::new(root).into()
  }

  fn create_main_menu_bar(root: XmlNode) -> UiComponentPtr {
    MainMenuBar::new(root).into()
  }

  fn create_menu(root: XmlNode) -> UiComponentPtr {
    Menu::new(root).into()
  }

  fn create_menu_item(root: XmlNode) -> UiComponentPtr {
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

pub type SubElementCreator = fn(XmlNode) -> UiComponentPtr;
pub type SubElementMap = HashMap<&'static str, SubElementCreator>;

pub trait UiElementParent {
  fn valid_children() -> &'static SubElementMap;
}

pub fn parse_children<E: UiElementParent>(root: XmlNode) -> Vec<UiComponentPtr> {
  let parse_child = |root: XmlNode| -> Option<UiComponentPtr> {
    if let Some(f) = E::valid_children().get(root.name.as_str()) {
      Some(f(root))
    } else {
      // todo some kind of error here
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

/* ---------------------------------------------------------------------------------------- */

/* Template */

pub struct UiTemplate {
  base_component: UiComponentPtr,
  model: Rc<dyn UiModel>,
}

impl Default for UiTemplate {
  fn default() -> Self {
    Self {
      base_component: EmptyUi::default().into(),
      model: Rc::new(StaticUi),
    }
  }
}

impl UiTemplate {
  pub fn new<L: Logger>(node: XmlNode, models: &UiModelArchive, logger: &L) -> Self {
    let mut root = UiTemplate::default();
    if let Some(model) = models.lookup(&node.name) {
      root.model = model.clone();
      for node in node.children {
        if let Some(f) = Self::valid_children().get(node.name.as_str()) {
          root.base_component = f(node);
        } else {
          logger.error(format!("ui type of '{}' is not valid", node.name));
        }
      }
    }

    root
  }

  pub fn create_component<L: Logger>(&self, logger: &L) -> UiPtr {
    let mut id_map = BTreeMap::new();
    let el = self.base_component.borrow().clone_ui(&mut id_map);
    let instance = self.model.new_instance().unwrap();
    let component = UiComponentInstance::new(el, instance, id_map);
    Box::new(component)
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

/* Component */

pub type UiElementPtr = Rc<RefCell<dyn UiElement>>;
pub type UiComponentPtr = Rc<RefCell<dyn UiComponent>>;

pub trait UiComponent: DynClone + UiElement {
  fn update(
    &mut self,
    _ui: &imgui::Ui<'_>,
    _instance: &mut dyn UiModelInstance,
    _game: &mut dyn Game,
  ) {
    panic!("'update' not implemented");
  }

  fn clone_ui(&self, id_map: &mut BTreeMap<String, UiElementPtr>) -> UiComponentPtr;
}

dyn_clone::clone_trait_object!(UiComponent);

/* Instance */

pub type UiPtr = Box<UiComponentInstance>;

pub struct UiComponentInstance {
  el: UiComponentPtr,
  instance: Box<dyn UiModelInstance>,
  element_id_mapping: BTreeMap<String, UiElementPtr>,
}

impl UiComponentInstance {
  fn new(
    el: UiComponentPtr,
    instance: Box<dyn UiModelInstance>,
    element_id_mapping: BTreeMap<String, UiElementPtr>,
  ) -> Self {
    Self {
      el,
      instance,
      element_id_mapping,
    }
  }

  fn update(&mut self, ui: &imgui::Ui<'_>, game: &mut dyn Game) {
    self
      .el
      .borrow_mut()
      .update(ui, self.instance.as_mut(), game);
  }
}

impl Document for UiComponentInstance {
  fn get_element_by_id(&mut self, id: &str) -> Option<Rc<RefCell<dyn UiElement>>> {
    self.element_id_mapping.get_mut(id).cloned()
  }
}

impl AsPtr for UiComponentInstance {}

/* Empty */

#[derive(Default, Clone)]
struct EmptyUi;

impl UiElement for EmptyUi {
  fn kind(&self) -> String {
    String::from("EmptyUi")
  }

  fn id(&self) -> Option<String> {
    None
  }

  fn set_attrib(&mut self, _attrib: String, _value: UiAttributeValue) {}
}

impl UiComponent for EmptyUi {
  fn clone_ui(&self, id_map: &mut BTreeMap<String, UiElementPtr>) -> UiComponentPtr {
    panic!("getting here is a logic error");
  }
}

impl Into<UiComponentPtr> for EmptyUi {
  fn into(self) -> UiComponentPtr {
    Rc::new(RefCell::new(self))
  }
}
