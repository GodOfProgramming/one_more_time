use super::common::*;

#[derive(Clone)]
pub struct MenuItem {
  name: CString,
  on_click: Option<String>,
}

impl MenuItem {
  pub fn new(mut root: XmlNode) -> Self {
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    let on_click = root.attribs.remove("click");

    Self { name, on_click }
  }
}

impl UiElement for MenuItem {
  fn update(&mut self, ui: &Ui<'_>, lua: Option<&Lua>, _settings: &Settings) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    if imgui::MenuItem::new(im_str).build(ui) {
      if let Some(on_click) = &self.on_click {
        if let Some(lua) = lua {
          let res: Result<(), mlua::Error> = lua.globals().call_function(on_click.as_str(), ());
          if let Err(e) = res {
            println!("error {}", e);
          }
        }
      }
    }
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
