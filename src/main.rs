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
const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const TITLE: &str = "Apidae";

// Controls struct
struct Controls {
  move_forward: bool,
  move_backward: bool,
  move_upward: bool,
  move_downward: bool,
  strafe_left: bool,
  strafe_right: bool,
  turn_left: bool,
  turn_right: bool,
}

// Controls implementation
impl Controls {
  // New function
  pub fn new() -> Self {
    Controls {
      move_forward: false,
      move_backward: false,
      move_upward: false,
      move_downward: false,
      strafe_left: false,
      strafe_right: false,
      turn_left: false,
      turn_right: false,
    }
  }

  // Set move forward state
  pub fn set_move_forward(&mut self, state: bool) {
    self.move_forward = state;
    println!("move_forward {}", Self::get_state_string(state));
  }

  // Set move backward state
  pub fn set_move_backward(&mut self, state: bool) {
    self.move_backward = state;
    println!("move_backward {}", Self::get_state_string(state));
  }

  // Set move upward state
  pub fn set_move_upward(&mut self, state: bool) {
    self.move_upward = state;
    println!("move_upward {}", Self::get_state_string(state));
  }

  // Set move downward state
  pub fn set_move_downward(&mut self, state: bool) {
    self.move_downward = state;
    println!("move_downward {}", Self::get_state_string(state));
  }

  // Set strafe left state
  pub fn set_strafe_left(&mut self, state: bool) {
    self.strafe_left = state;
    println!("strafe_left {}", Self::get_state_string(state));
  }

  // Set strafe right state
  pub fn set_strafe_right(&mut self, state: bool) {
    self.strafe_right = state;
    println!("strafe_right {}", Self::get_state_string(state));
  }

  // Set turn left state
  pub fn set_turn_left(&mut self, state: bool) {
    self.turn_left = state;
    println!("turn_left {}", Self::get_state_string(state));
  }

  // Set turn right state
  pub fn set_turn_right(&mut self, state: bool) {
    self.turn_right = state;
    println!("turn_right {}", Self::get_state_string(state));
  }

  // Get move forward state
  pub fn _get_move_forward(&mut self) -> bool {
    self.move_forward
  }

  // Get move backward state
  pub fn _get_move_backward(&mut self) -> bool {
    self.move_backward
  }

  // Get move upward state
  pub fn _get_move_upward(&mut self) -> bool {
    self.move_upward
  }

  // Get move downward state
  pub fn _get_move_downward(&mut self) -> bool {
    self.move_downward
  }

  // Get strafe left state
  pub fn _get_strafe_left(&mut self) -> bool {
    self.strafe_left
  }

  // Get strafe right state
  pub fn _get_strafe_right(&mut self) -> bool {
    self.strafe_right
  }

  // Get turn left state
  pub fn _get_turn_left(&mut self) -> bool {
    self.turn_left
  }

  // Get turn right state
  pub fn _get_turn_right(&mut self) -> bool {
    self.turn_right
  }

  // Get state string from boolean
  fn get_state_string(state: bool) -> &'static str {
    if state { "ON" } else { "OFF" }
  }
}

// App struct
struct App {
  gl: GlGraphics,
  ctrl: Controls,
}

// App implementation
impl App {
  // Control function
  fn control(&mut self, args: &ButtonArgs) {
    let state: bool = args.state == ButtonState::Press{};
    match args.button {
      Button::Keyboard(Key::Space) => self.ctrl.set_move_upward(state),
      Button::Keyboard(Key::A) => self.ctrl.set_strafe_left(state),
      Button::Keyboard(Key::D) => self.ctrl.set_strafe_right(state),
      Button::Keyboard(Key::E) => self.ctrl.set_turn_right(state),
      Button::Keyboard(Key::W) => self.ctrl.set_move_forward(state),
      Button::Keyboard(Key::Q) => self.ctrl.set_turn_left(state),
      Button::Keyboard(Key::S) => self.ctrl.set_move_backward(state),
      Button::Keyboard(Key::LCtrl) => self.ctrl.set_move_downward(state),
      _ => (),
    }
  }

  // Update function
  fn update(&mut self, _args: &UpdateArgs) {

  }

  // Render function
  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;
    self.gl.draw(args.viewport(), |c, gl| {
      clear([0.05, 0.05, 0.05, 1.0], gl);
      rectangle([0.95, 0.95, 0.95, 1.0], rectangle::square(WIDTH / 2.0, HEIGHT / 2.0, 2.0), c.transform, gl);
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
    ctrl: Controls::new(),
  };

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
