use super::common::*;

#[derive(Clone)]
pub struct Window {
  title: CString,
  transparent: bool,
  decorated: bool,
  children: UiSubElements,
}

impl Window {
  pub fn new(mut root: XmlNode) -> Self {
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
      title,
      transparent,
      decorated,
      children: super::parse_children::<Self>(root),
    }
  }
}

impl UiElement for Window {
  fn update(&mut self, ui: &Ui<'_>, lua: Option<&Lua>, settings: &Settings) {
    let im_str = unsafe { ImStr::from_cstr_unchecked(&self.title) };
    let children = &mut self.children;
    imgui::Window::new(&im_str)
      .save_settings(false)
      .focus_on_appearing(false)
      .bg_alpha(if self.transparent { 0.0 } else { 1.0 })
      .build(ui, || {
        for child in children.iter_mut() {
          child.update(ui, lua, settings);
        }
      });
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
