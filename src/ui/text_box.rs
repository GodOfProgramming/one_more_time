use super::common::*;

#[derive(Clone)]
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
  fn update(&mut self, ui: &imgui::Ui<'_>, _lua: Option<&Lua>, _settings: &Settings) {
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

impl Into<Ui> for TextBox {
  fn into(self) -> Ui {
    Ui(Rc::new(RefCell::new(self)))
  }
}
