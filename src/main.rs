mod game;
mod gfx;
mod input;
mod math;
mod ui;
mod util;
mod view;

use game::App;

fn main() {
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let isolate = &mut v8::Isolate::new(Default::default());

  let handle_scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(handle_scope);
  let scope = &mut v8::ContextScope::new(handle_scope, context);

  fn foo(
    a: &mut v8::HandleScope<'_>,
    b: v8::FunctionCallbackArguments<'_>,
    mut c: v8::ReturnValue<'_>,
  ) {
    let s = b.get(0);
    println!("{}", s.to_rust_string_lossy(a));
    let v = v8::Integer::new(a, b.length());
    let v: v8::Local<v8::Value> = v8::Local::from(v);
    c.set(v);
  }

  let global = context.global(scope);

  let name: v8::Local<v8::Value> = v8::Local::from(v8::String::new(scope, "foo").unwrap());
  let function: v8::Local<v8::Value> = v8::Local::from(v8::Function::new(scope, foo).unwrap());
  global.set(scope, name, function);

  let code = v8::String::new(scope, "foo('bar');").unwrap();
  println!("javascript code: {}", code.to_rust_string_lossy(scope));

  let script = v8::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));

  std::process::exit(0);

  // let mut app = App::new();

  // app.run();
}
