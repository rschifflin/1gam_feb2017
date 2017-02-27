use specs::{Component, VecStorage};
use fsm::bird::BirdFSM;
use progress::Progress;

#[derive(Debug)]
pub struct Bird {
  state: BirdFSM,
  requisite_progress: Progress,
  pub reward_progress: Progress,
}

impl Component for Bird {
  type Storage = VecStorage<Bird>;
}

impl Bird {
  pub fn new(requisite_progress: Progress, reward_progress: Progress) -> Bird {
    Bird {
      state: BirdFSM::new(),
      requisite_progress: requisite_progress,
      reward_progress: reward_progress
    }
  }

  pub fn heard_song(&mut self, song: Progress) {
    if song.contains(self.requisite_progress) {
      self.state = self.state.on_heard_song();
    }
  }

  pub fn update(&mut self) {
    self.state = self.state.update();
  }

  pub fn should_sing(&self) -> bool {
    self.state.should_sing()
  }
}
