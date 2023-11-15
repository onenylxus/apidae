// Controls struct
pub(crate) struct Controls {
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
