#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate cpal;
#[macro_use] extern crate conrod;
extern crate float;
extern crate futures;
extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
extern crate specs;
extern crate collider;

mod components;
mod systems;
#[allow(dead_code)] mod colors;
/*
mod sound;
mod screen;
*/
mod world;
/*
mod ui;
*/


use piston::window::WindowSettings;
use piston::input::*;
use glium_graphics::{Glium2d, GliumGraphics, OpenGL, GliumWindow};
use glium::{Surface, Frame};
use specs::{/*RunArg,*/ Join, World};
//use futures::sync::mpsc::channel;
//use sound::{SoundEvent, spawn_audio_thread};

pub struct App {
  //ui: ui::Ui, //Conrod drawing context
  gl: Glium2d, // OpenGL drawing backend.
  context: world::Context,
  planner: specs::Planner<world::Context>,
  time_since_update: f64
}

impl App {
  fn render(&mut self, args: &RenderArgs, window: &mut glium_graphics::GliumWindow) {
    let world = self.planner.mut_world();
    //let ui = &mut self.ui;
    let mut frame = window.draw();
    frame.clear_color(1.0, 0.0, 0.0, 1.0);
    self.gl.draw(&mut frame, args.viewport(), |c, g| {
      //ui.draw(c, g);
      Self::render_gfx(c, g, world);
    });

    frame.finish().unwrap();
  }

  fn render_gfx(c: graphics::Context, g: &mut GliumGraphics<Frame>, world: &mut World) {
    use graphics::{ellipse, rectangle, Transformed};

    let positions = world.read::<components::Position>();
    let sprites = world.read::<components::Sprite>();
    for (pos, sprite) in (&positions, &sprites).iter() {
      let xform = c.transform.trans(pos.x, pos.y);
      rectangle(colors::GREEN, [0.0, 0.0, 10.0, 10.0], xform, g);
    };
  }

  fn input(&mut self, args: &Input) {
    let p1_direction = match *args {
      Input::Resize(w, h) => println!("Resizing: {}, {}", w, h),
      _ => ()
    };

    /*self.ui.ui.handle_event(args.clone());

    use components::control::player::Direction;
    let Context { p1_paddle, p2_paddle, .. } = self.context.clone();

    let p1_direction = match *args {
      Input::Press(Button::Keyboard(keyboard::Key::S)) => Some(Direction::Down),
      Input::Press(Button::Keyboard(keyboard::Key::W)) => Some(Direction::Up),
      Input::Release(Button::Keyboard(keyboard::Key::S)) => Some(Direction::Neutral),
      Input::Release(Button::Keyboard(keyboard::Key::W)) => Some(Direction::Neutral),
      _ => None
    };

    let p2_direction = match *args {
      Input::Press(Button::Keyboard(keyboard::Key::Down)) => Some(Direction::Down),
      Input::Press(Button::Keyboard(keyboard::Key::Up)) => Some(Direction::Up),
      Input::Release(Button::Keyboard(keyboard::Key::Down)) => Some(Direction::Neutral),
      Input::Release(Button::Keyboard(keyboard::Key::Up)) => Some(Direction::Neutral),
      _ => None
    };

    self.planner.run_custom(move |arg: RunArg| {
      let mut players = arg.fetch(|w| {
        w.write::<::components::control::Player>()
      });

      p1_direction.and_then(|dir| {
        players.get_mut(p1_paddle).map(|mut p1| {
          p1.direction = dir;
        })
      });

      p2_direction.and_then(|dir| {
        players.get_mut(p2_paddle).map(|mut p2| {
          p2.direction = dir;
        })
      });
    })
    */
  }

  fn update(&mut self, &UpdateArgs { dt }: &UpdateArgs) {
    self.time_since_update += dt;
    if self.time_since_update > 0.0166666 {
      self.planner.dispatch(self.context.clone());
      self.time_since_update = 0.0;
    }
  }
}

fn main() {
  /*
  let (sound_tx, sound_rx) = channel::<SoundEvent>(0);
  spawn_audio_thread(sound_rx);
  */

  let opengl = OpenGL::V3_2;
  let mut window: GliumWindow = WindowSettings::new(
    "Noteworthy",
    [640, 480]
  )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new game and run it.
  let mut world = specs::World::new();
  world::register(&mut world);
  let (director,) = world::create_initial_entities(&mut world);
  let context = world::Context {
    director: director
    //sound_tx: sound_tx
  };
  let mut planner = specs::Planner::<world::Context>::new(world, 4);
  systems::plan_system(&mut planner, systems::physics::Physics, 0);
  /*
  systems::plan_system(&mut planner, systems::control::Player, 0);
    let mut ui = ui::Ui::new(&mut window);
    ui.update();
  */

  let mut app = App {
    //ui: ui,
    gl: Glium2d::new(opengl, &window),
    context: context,
    planner: planner,
    time_since_update: 0.0
  };

  while let Some(e) = window.next() {
    if let &Event::Input(ref i) = &e {
      app.input(i);
    }
    e.update(|args| app.update(&args));
    e.render(|args| {
      app.render(&args, &mut window);
    });
  }
}
