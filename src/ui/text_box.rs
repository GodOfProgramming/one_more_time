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

  fn set_attrib(&mut self, _: String, _: UiAttributeValue) {
    // do nothing for now
  }
}

impl UiComponent for TextBox {
  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    instance: &mut dyn UiModelInstance,
    _settings: &Settings,
  ) {
    ui.text(&self.text);
  }

  fn clone_ui(&self, id_map: &mut BTreeMap<String, UiElementPtr>) -> UiComponentPtr {
    let ui = Rc::new(RefCell::new(self.clone()));
    let ptr: Rc<RefCell<dyn UiElement>> = ui.clone();

    if let Some(id) = self.id() {
      id_map.insert(id, ptr);
    }

    ui
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

impl From<TextBox> for UiComponentPtr {
  fn from(ui: TextBox) -> Self {
    Rc::new(RefCell::new(ui))
  }
}
