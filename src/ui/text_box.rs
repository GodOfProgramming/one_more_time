use super::common::*;

#[derive(Clone)]
pub struct TextBox {
  id: Option<String>,
  text: String,
}

impl TextBox {
  pub fn new(mut root: XmlNode) -> Self {
    let id = root.attribs.remove("id");
    let text = root.text.unwrap_or_default();

    Self { id, text }
  }
}

impl UiElement for TextBox {
  fn kind(&self) -> String {
    String::from("TextBox")
  }

  fn id(&self) -> Option<String> {
    self.id.clone()
  }

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    class: &LuaValue,
    instance: &LuaValue,
    _settings: &Settings,
  ) {
    ui.text(&self.text);
  }

  fn dupe(&self) -> UiElementPtr {
    Rc::new(RefCell::new(self.clone()))
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

impl From<TextBox> for Ui {
  fn from(ui: TextBox) -> Self {
    Ui(Rc::new(RefCell::new(ui)))
  }
}
