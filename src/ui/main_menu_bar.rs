use super::common::*;

#[derive(Clone)]
pub struct MainMenuBar {
  children: Vec<Ui>,
}

impl MainMenuBar {
  pub fn new(root: XmlNode) -> Self {
    Self {
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for MainMenuBar {
  fn update(&mut self, ui: &imgui::Ui<'_>, lua: Option<&Lua>, settings: &Settings) {
    ui.main_menu_bar(|| {
      for child in self.children.iter_mut() {
        child.update(ui, lua, settings);
      }
    });
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

impl Into<Ui> for MainMenuBar {
  fn into(self) -> Ui {
    Ui(Rc::new(RefCell::new(self)))
  }
}
