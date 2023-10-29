// Extern crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// Use dependencies
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

// App constants
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const TITLE: &str = "Apidae";

// App struct
struct App {
  gl: GlGraphics,
}

// App implementation
impl App {
  // Update function
  fn update(&mut self, _args: &UpdateArgs) {

  }

  // Render function
  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;
    self.gl.draw(args.viewport(), |_c, gl| {
      clear([0.05, 0.05, 0.05, 1.0], gl);
    })
  }
}

// Main function
fn main() {
  // Select OpenGL version
  let opengl = OpenGL::V3_2;

  // Create window
  let mut window: Window = WindowSettings::new(TITLE, [WIDTH, HEIGHT])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create application
  let mut app: App = App {
    gl: GlGraphics::new(opengl),
  };

  // Create event iterator
  let mut events = Events::new(EventSettings::new());

  // Handle events
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.update_args() {
      app.update(&args);
    }
    if let Some(args) = e.render_args() {
      app.render(&args);
    }
  }
}
