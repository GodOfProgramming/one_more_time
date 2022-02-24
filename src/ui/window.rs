use super::common::*;

#[derive(Clone)]
pub struct Window {
  id: Option<String>,
  title: CString,
  transparent: bool,
  decorated: bool,
  children: Vec<UiComponentPtr>,
}

impl Window {
  pub fn new(mut root: XmlNode) -> Self {
    let id = root.attribs.remove("id");

    let title = root
      .attribs
      .remove("title")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    let transparent = root
      .attribs
      .get("transparent")
      .and_then(|v| v.parse::<bool>().ok())
      .unwrap_or_default();

    let decorated = root
      .attribs
      .get("decorated")
      .and_then(|v| v.parse::<bool>().ok())
      .unwrap_or_default();

    Self {
      id,
      title,
      transparent,
      decorated,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for Window {
  fn kind(&self) -> String {
    String::from("Window")
  }

  fn id(&self) -> Option<String> {
    self.id.clone()
  }

  fn set_attrib(&mut self, attrib: String, value: UiAttributeValue) {
    match attrib.as_str() {
      "title" => {
        if let UiAttributeValue::String(s) = value {
          self.title = string::into_cstring(s.as_str());
        }
      }
      "transparent" => {
        if let UiAttributeValue::Bool(b) = value {
          self.transparent = b;
        }
      }
      "decorated" => {
        if let UiAttributeValue::Bool(b) = value {
          self.decorated = b;
        }
      }
      _ => (),
    }
  }
}

impl UiComponent for Window {
  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    instance: &mut dyn UiModelInstance,
    settings: &Settings,
  ) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.title) };
    let children = &mut self.children;
    imgui::Window::new(&im_str)
      .save_settings(false)
      .focus_on_appearing(false)
      .bg_alpha(if self.transparent { 0.0 } else { 1.0 })
      .build(ui, || {
        for child in children.iter_mut() {
          child.borrow_mut().update(logger, ui, instance, settings);
        }
      });
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

impl UiElementParent for Window {
  fn valid_children() -> &'static SubElementMap {
    use types::TEXTBOX;

    lazy_static! {
      static ref MAP: SubElementMap = type_map![TEXTBOX];
    }

    &MAP
  }
}

impl From<Window> for UiComponentPtr {
  fn from(ui: Window) -> Self {
    Rc::new(RefCell::new(ui))
  }
}

impl AsPtr for Window {}
