use super::common::*;

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
  fn update(&mut self, ui: &Ui<'_>, lua: Option<&Lua>, _settings: &Settings) {
    ui.text(&self.text);
  }
}

impl UiElementParent for TextBox {
  fn valid_children() -> &'static SubElementMap {
    lazy_static! {
      static ref MAP: SubElementMap = type_map![];
    }

    &MAP
  }
}
