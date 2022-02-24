use super::common::*;

#[derive(Clone)]
pub struct MainMenuBar {
  id: Option<String>,
  children: Vec<UiComponentPtr>,
}

impl MainMenuBar {
  pub fn new(mut root: XmlNode) -> Self {
    let id = root.attribs.remove("id");
    Self {
      id,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for MainMenuBar {
  fn kind(&self) -> String {
    String::from("MainMenuBar")
  }

  fn id(&self) -> Option<String> {
    self.id.clone()
  }

  fn set_attrib(&mut self, _: String, _: UiAttributeValue) {
    // do nothing for now
  }
}

impl UiComponent for MainMenuBar {
  fn update(
    &mut self,
    ui: &imgui::Ui<'_>,
    instance: &mut dyn UiModelInstance,
    game: &mut dyn Game,
  ) {
    ui.main_menu_bar(|| {
      for child in self.children.iter_mut() {
        child.borrow_mut().update(ui, instance, game);
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

impl UiElementParent for MainMenuBar {
  fn valid_children() -> &'static SubElementMap {
    use types::MENU;
    lazy_static! {
      static ref MAP: SubElementMap = type_map![MENU];
    }

    &MAP
  }
}

impl From<MainMenuBar> for UiComponentPtr {
  fn from(ui: MainMenuBar) -> Self {
    Rc::new(RefCell::new(ui))
  }
}
