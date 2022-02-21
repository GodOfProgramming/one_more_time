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

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    class: &LuaValue,
    instance: &LuaValue,
    _settings: &Settings,
  ) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    if imgui::MenuItem::new(im_str).build(ui) {
      if let Some(on_click) = &self.on_click {
        if let LuaValue::Table(class) = class {
          let res: mlua::Result<()> = class.call_function(on_click.as_str(), instance.clone());
          if let Err(e) = res {
            logger.error(e.to_string());
          }
        }
      }
    }
  }

  fn dupe(&self) -> UiElementPtr {
    Box::new(self.clone())
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

impl From<MenuItem> for Ui {
  fn from(ui: MenuItem) -> Self {
    Ui(Box::new(ui))
  }
}
