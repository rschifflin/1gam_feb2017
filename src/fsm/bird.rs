const LISTEN_TIME: usize = 60;

#[derive(Debug, Copy, Clone)]
pub enum BirdState {
  Idle,
  Listening,
  Singing,
  Done
}

#[derive(Debug, Copy, Clone)]
pub struct BirdFSM {
  bird_state: (usize, BirdState)
}

impl BirdFSM {
  pub fn new() -> BirdFSM {
    BirdFSM {
      bird_state: (0, BirdState::Idle)
    }
  }

  pub fn on_heard_song(&self) -> BirdFSM {
    match self.bird_state {
      (_, BirdState::Idle) => {
        BirdFSM {
          bird_state: (0, BirdState::Listening)
        }
      },
      _ => *self
    }
  }

  pub fn should_sing(&self) -> bool {
    match self.bird_state {
      (_, BirdState::Singing) => true,
      _ => false
    }
  }

  pub fn update(&self) -> BirdFSM {
    match self.bird_state {
      (_, BirdState::Idle) => {
        BirdFSM {
          bird_state: (0, BirdState::Idle)
        }
      },
      (n, BirdState::Listening) => {
        if n >= LISTEN_TIME {
          BirdFSM {
            bird_state: (0, BirdState::Singing)
          }
        } else {
          BirdFSM {
            bird_state: (n+1, BirdState::Listening)
          }
        }
      },
      _ => {
        BirdFSM {
          bird_state: (0, BirdState::Done)
        }
      }
    }
  }
}

