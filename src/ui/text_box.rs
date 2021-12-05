use super::{SubElementMap, UiElement, UiElementParent};
use crate::{
  type_map,
  util::{Settings, XmlNode},
};
use imgui_glium_renderer::imgui::Ui;
use maplit::hashmap;

pub struct TextBox {
  text: String,
}

impl TextBox {
  pub fn new(root: XmlNode) -> Self {
    let text = root.text.unwrap_or_default();

    Self { text }
  }
}

impl UiElement for TextBox {
  fn update(&mut self, ui: &Ui<'_>, _settings: &Settings) {
    ui.text(&self.text);
  }
}

impl UiElementParent for TextBox {
  fn valid_children() -> SubElementMap {
    type_map![]
  }
}
