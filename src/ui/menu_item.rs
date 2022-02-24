use super::common::*;

#[derive(Clone)]
pub struct MenuItem {
  id: Option<String>,
  name: CString,
  on_click: Option<String>,
}

impl MenuItem {
  pub fn new(mut root: XmlNode) -> Self {
    let id = root.attribs.remove("id");
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    let on_click = root.attribs.remove("click");

    Self { id, name, on_click }
  }
}

impl UiElement for MenuItem {
  fn kind(&self) -> String {
    String::from("MenuItem")
  }

  fn id(&self) -> Option<String> {
    self.id.clone()
  }

  fn set_attrib(&mut self, _: String, _: UiAttributeValue) {
    // do nothing for now
  }
}

impl UiComponent for MenuItem {
  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    instance: &mut dyn UiModelInstance,
    _settings: &Settings,
  ) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    if imgui::MenuItem::new(im_str).build(ui) {
      if let Some(on_click) = &self.on_click {
        instance.call_handler(on_click);
      }
    }
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

impl UiElementParent for MenuItem {
  fn valid_children() -> &'static SubElementMap {
    lazy_static! {
      static ref MAP: SubElementMap = type_map![];
    }

    &MAP
  }
}

impl From<MenuItem> for UiComponentPtr {
  fn from(ui: MenuItem) -> Self {
    Rc::new(RefCell::new(ui))
  }
}
