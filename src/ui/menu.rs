use super::common::*;

#[derive(Clone)]
pub struct Menu {
  id: Option<String>,
  name: CString,
  children: Vec<Ui>,
}

impl Menu {
  pub fn new(mut root: XmlNode) -> Self {
    let id = root.attribs.remove("id");
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    Self {
      id,
      name,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for Menu {
  fn kind(&self) -> String {
    String::from("Menu")
  }

  fn id(&self) -> Option<String> {
    self.id.clone()
  }

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    lua: Option<&Lua>,
    settings: &Settings,
  ) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    let children = &mut self.children;
    if let Some(menu) = ui.begin_menu(im_str) {
      for child in children.iter_mut() {
        child.update(logger, ui, lua, settings);
      }
      menu.end();
    }
  }

  fn dupe(&self) -> UiElementPtr {
    Rc::new(RefCell::new(self.clone()))
  }
}

impl UiElementParent for Menu {
  fn valid_children() -> &'static SubElementMap {
    use types::MENU_ITEM;
    lazy_static! {
      static ref MAP: SubElementMap = type_map![MENU_ITEM];
    }

    &MAP
  }
}

impl From<Menu> for Ui {
  fn from(ui: Menu) -> Self {
    Ui(Rc::new(RefCell::new(ui)))
  }
}
