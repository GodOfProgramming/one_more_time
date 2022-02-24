pub use chlorine;
pub use chrono;
pub use double;
pub use dyn_clone;
pub use fern;
pub use glfw;
pub use image;
pub use imgui_glium_renderer::{self, glium, imgui};
pub use lazy_static;
pub use libloading;
pub use log;
pub use maplit;
pub use nalgebra_glm as glm;
pub use ncollide2d;
pub use profiling;
pub use puffin;
pub use puffin_imgui;
pub use regex;
pub use scheduled_thread_pool;
pub use toml;
pub use uid;
pub use walkdir;
pub use xml;

pub mod core;
pub mod ui;
pub mod util;

pub struct Plugin;

pub enum PluginLoadError {
  GeneralFailure(String),
}

pub type PluginResult = Result<Plugin, PluginLoadError>;
