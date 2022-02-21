use super::common::*;

#[derive(Clone)]
pub struct MainMenuBar {
  id: Option<String>,
  children: Vec<Ui>,
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

  fn update(
    &mut self,
    logger: &dyn Logger,
    ui: &imgui::Ui<'_>,
    class: &LuaValue,
    instance: &LuaValue,
    settings: &Settings,
  ) {
    ui.main_menu_bar(|| {
      for child in self.children.iter_mut() {
        child.update(logger, ui, class, instance, settings);
      }
    });
  }

  fn dupe(&self) -> UiElementPtr {
    Rc::new(RefCell::new(self.clone()))
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

impl From<MainMenuBar> for Ui {
  fn from(ui: MainMenuBar) -> Self {
    Ui(Rc::new(RefCell::new(ui)))
  }
}
