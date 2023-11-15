// Use dependencies
use opengl_graphics::GlGraphics;
use piston::input::*;
use super::controls::Controls;

// App struct
pub(crate) struct App {
  gl: GlGraphics,
  ctrl: Controls,
}

// App implementation
impl App {
  // New function
  pub fn new() -> Self {
    App {
      gl: GlGraphics::new(crate::OPENGL),
      ctrl: Controls::new(),
    }
  }

  // Control function
  pub fn control(&mut self, args: &ButtonArgs) {
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
  pub fn update(&mut self, _args: &UpdateArgs) {

  }

  // Render function
  pub fn render(&mut self, args: &RenderArgs) {
    use graphics::*;
    self.gl.draw(args.viewport(), |c, gl| {
      clear([0.05, 0.05, 0.05, 1.0], gl);
      rectangle([0.95, 0.95, 0.95, 1.0], rectangle::square(crate::WIDTH / 2.0 - 1.0, crate::HEIGHT / 2.0 - 1.0, 2.0), c.transform, gl);
    })
  }
}
