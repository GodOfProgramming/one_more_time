use super::{types, SubElementMap, UiElement, UiElementParent, UiSubElements};
use crate::{
  type_map,
  util::{convert::string, Settings, XmlNode},
};
use imgui_glium_renderer::imgui::{ImStr, Ui};
use maplit::hashmap;
use std::ffi::CString;

pub struct Menu {
  name: CString,
  children: UiSubElements,
}

impl Menu {
  pub fn new(mut root: XmlNode) -> Self {
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    Self {
      name,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for Menu {
  fn update(&mut self, ui: &Ui<'_>, settings: &Settings) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    let children = &mut self.children;
    if let Some(menu) = ui.begin_menu(im_str) {
      for child in children.iter_mut() {
        child.update(ui, settings);
      }
      menu.end();
    }
  }
}

impl UiElementParent for Menu {
  fn valid_children() -> SubElementMap {
    use types::MENU_ITEM;
    type_map![MENU_ITEM]
  }
}
