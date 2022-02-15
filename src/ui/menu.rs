use super::common::*;

#[derive(Clone)]
pub struct Menu {
  name: CString,
  children: UiSubElements,
}

impl Menu {
  pub fn new(mut root: XmlNode) -> Self {
    let name = root
      .attribs
      .remove("name")
      .map(|v| string::into_cstring(&v))
      .unwrap_or_default();

    Self {
      name,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for Menu {
  fn update(&mut self, ui: &Ui<'_>, lua: Option<&Lua>, settings: &Settings) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    let children = &mut self.children;
    if let Some(menu) = ui.begin_menu(im_str) {
      for child in children.iter_mut() {
        child.update(ui, lua, settings);
      }
      menu.end();
    }
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
