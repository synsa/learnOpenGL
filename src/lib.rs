#![deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate gl;
extern crate cgmath;
extern crate time;
extern crate image;
extern crate wavefront_obj;


mod result;
mod app;
mod painter;
mod assets;
mod camera;
mod obj;
mod model;
pub mod math;

pub use result::{AppError, Result};
pub use app::App;
pub use painter::{Painter, Api};
pub use assets::{load_program, load_cubemap, load_obj};
pub use model::Model;
pub use camera::Camera;
pub use obj::Obj;