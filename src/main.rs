mod game;
mod gfx;
mod input;
mod math;
mod ui;
mod util;
mod view;

use game::App;
use mlua::{prelude::*, StdLib, UserData, UserDataFields};
use std::cell::RefCell;
use std::rc::Rc;
use util::MainLogger;

struct Test {
  pub id: i32,
}

impl Test {
  pub fn test(&self) {
    println!("id = {}", self.id);
  }
}

struct LuaObj<T>(Rc<RefCell<T>>);

impl LuaObj<Test> {
  pub fn new(id: i32) -> Self {
    Self(Rc::new(RefCell::new(Test { id })))
  }

  pub fn id(&self) -> i32 {
    self.0.borrow().id
  }

  pub fn set_id(&mut self, id: i32) {
    self.0.borrow_mut().id = id;
  }
}

impl Clone for LuaObj<Test> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl UserData for LuaObj<Test> {
  fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("id", |_, this| Ok(this.id()));
    fields.add_field_method_set("id", |_, this, val: i32| {
      this.set_id(val);
      Ok(())
    });
  }
}

fn main() {
  let lua: Lua = Lua::new();

  let tbl = lua.create_table().unwrap();
  let test = LuaObj::<Test>::new(1);
  let ud = lua.create_userdata(test.clone()).unwrap();
  tbl.set("data", ud).unwrap();
  lua.globals().set("gbl", tbl).unwrap();

  lua
    .load("print(gbl.data.id); gbl.data.id = 2;")
    .exec()
    .unwrap();

  println!("id = {}", test.id());

  let mut app = App::new();

  app.run();
}
