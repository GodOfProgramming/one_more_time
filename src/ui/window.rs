use super::common::*;

#[derive(Clone)]
pub struct Window {
  id: Option<String>,
  title: CString,
  transparent: bool,
  decorated: bool,
  children: Vec<Ui>,
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

  fn set_attrib(&mut self, attrib: String, value: Value) {
    match attrib.as_str() {
      "title" => {
        if let Value::String(s) = value {
          self.title = string::into_cstring(s.to_str().unwrap());
        }
      }
      "transparent" => {
        if let Value::Boolean(b) = value {
          self.transparent = b;
        }
      }
      "decorated" => {
        if let Value::Boolean(b) = value {
          self.decorated = b;
        }
      }
      _ => (),
    }
  }

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    class: &LuaValue,
    instance: &LuaValue,
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
          child.update(logger, ui, class, instance, settings);
        }
      });
  }

  fn dupe(&self) -> UiElementPtr {
    Box::new(self.clone())
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

impl From<Window> for Ui {
  fn from(ui: Window) -> Self {
    Ui(Box::new(ui))
  }
}
