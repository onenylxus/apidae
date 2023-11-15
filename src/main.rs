// Extern crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Use modules
mod kernel;

// Use dependencies
use glutin_window::GlutinWindow as Window;
use kernel::app::App;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

// App constants
const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const TITLE: &str = "Apidae";
const OPENGL: OpenGL = OpenGL::V3_2;

// Main function
fn main() {
  // Create window
  let mut window: Window = WindowSettings::new(TITLE, [WIDTH, HEIGHT])
    .graphics_api(OPENGL)
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create application
  let mut app: App = App::new();

  // Create event iterator
  let mut events = Events::new(EventSettings::new());

  // Handle events
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.button_args() {
      app.control(&args);
    }
    if let Some(args) = e.update_args() {
      app.update(&args);
    }
    if let Some(args) = e.render_args() {
      app.render(&args);
    }
  }
}
