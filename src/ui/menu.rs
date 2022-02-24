use super::common::*;

#[derive(Clone)]
pub struct Menu {
  id: Option<String>,
  name: CString,
  children: Vec<UiComponentPtr>,
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

  fn set_attrib(&mut self, _: String, _: UiAttributeValue) {
    // do nothing for now
  }
}

impl UiComponent for Menu {
  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    instance: &mut dyn UiModelInstance,
    settings: &Settings,
  ) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.name) };
    let children = &mut self.children;
    if let Some(menu) = ui.begin_menu(im_str) {
      for child in children.iter_mut() {
        child.borrow_mut().update(logger, ui, instance, settings);
      }
      menu.end();
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

impl UiElementParent for Menu {
  fn valid_children() -> &'static SubElementMap {
    use types::MENU_ITEM;
    lazy_static! {
      static ref MAP: SubElementMap = type_map![MENU_ITEM];
    }

    &MAP
  }
}

impl From<Menu> for UiComponentPtr {
  fn from(ui: Menu) -> Self {
    Rc::new(RefCell::new(ui))
  }
}
