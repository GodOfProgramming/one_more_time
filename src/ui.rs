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

use crate::util::{Logger, Settings, XmlNode};
use imgui_glium_renderer::imgui::Ui;
use lazy_static::lazy_static;
use log::warn;
use maplit::hashmap;
use std::collections::HashMap;
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

type UiSubElement = Box<dyn UiElement + Send + Sync>;
type UiSubElements = Vec<UiSubElement>;
type SubElementCreator = fn(XmlNode) -> UiSubElement;
type SubElementMap = HashMap<&'static str, SubElementCreator>;

pub trait UiElement {
  fn update(&mut self, ui: &Ui<'_>, settings: &Settings);
}

pub trait UiElementParent {
  fn valid_children() -> &'static SubElementMap;
}

struct EmptyUi;

impl UiElement for EmptyUi {
  fn update(&mut self, _: &Ui<'_>, _: &Settings) {
    panic!("'update' unimplemented for EmptyUi")
  }
}

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

pub struct UiRoot {
  el: UiSubElement,
}

impl Default for UiRoot {
  fn default() -> Self {
    Self {
      el: Box::new(EmptyUi),
    }
  }
}

impl UiElement for UiRoot {
  fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    self.el.update(ui, settings);
  }
}

impl UiElementParent for UiRoot {
  fn valid_children() -> &'static SubElementMap {
    use types::{MAIN_MENU_BAR, WINDOW};

    lazy_static! {
      static ref MAP: SubElementMap = type_map![WINDOW, MAIN_MENU_BAR];
    }

    &MAP
  }
}

impl<L: Logger> From<(&L, XmlNode)> for UiRoot {
  fn from((logger, node): (&L, XmlNode)) -> Self {
    let mut root = UiRoot::default();

    if let Some(f) = Self::valid_children().get(node.name.as_str()) {
      root.el = f(node);
    } else {
      logger.warn(format!("ui type of '{}' is not valid", node.name));
    }

    root
  }
}
