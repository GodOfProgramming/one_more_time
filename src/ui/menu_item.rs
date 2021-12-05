use super::{SubElementMap, UiElement, UiElementParent};
use crate::{
  type_map,
  util::{convert::string, Settings, XmlNode},
};
use imgui_glium_renderer::imgui::{self, ImStr, Ui};
use log::info;
use maplit::hashmap;
use std::ffi::CString;

pub struct MenuItem {
  name: CString,
}

impl MenuItem {
  pub fn new(mut root: XmlNode) -> Self {
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    Self { name }
  }
}

impl UiElement for MenuItem {
  fn update(&mut self, ui: &Ui<'_>, _settings: &Settings) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    if imgui::MenuItem::new(im_str).build(ui) {
      info!("click");
    }
  }
}

impl UiElementParent for MenuItem {
  fn valid_children() -> SubElementMap {
    type_map![]
  }
}
