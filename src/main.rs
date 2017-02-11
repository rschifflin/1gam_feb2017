#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate cpal;
#[macro_use] extern crate conrod;
#[macro_use] extern crate bitflags;
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
mod input;
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
    /*
      Input::Resize(w, h) => println!("Resizing: {}, {}", w, h),
    */

    /*self.ui.ui.handle_event(args.clone());

    use components::control::player::Direction;
    let Context { p1_paddle, p2_paddle, .. } = self.context.clone();
    */

    let mut current_input = self.context.input.0;
    match *args {
      /*
      Input::Press(x) => println!("Pressed {:?}", x),
      Input::Release(x) => println!("Released {:?}", x),
      */
      Input::Press(Button::Keyboard(keyboard::Key::Up)) => current_input = current_input | input::UP,
      Input::Press(Button::Keyboard(keyboard::Key::Down)) => current_input = current_input | input::DOWN,
      Input::Press(Button::Keyboard(keyboard::Key::Left)) => current_input = current_input | input::LEFT,
      Input::Press(Button::Keyboard(keyboard::Key::Right)) => current_input = current_input | input::RIGHT,
      Input::Press(Button::Keyboard(keyboard::Key::Space)) => current_input = current_input | input::JUMP,
      Input::Press(Button::Keyboard(keyboard::Key::Z)) => current_input = current_input | input::WHISTLE,

      Input::Release(Button::Keyboard(keyboard::Key::Up)) => current_input = current_input - input::UP,
      Input::Release(Button::Keyboard(keyboard::Key::Down)) => current_input = current_input - input::DOWN,
      Input::Release(Button::Keyboard(keyboard::Key::Left)) => current_input = current_input - input::LEFT,
      Input::Release(Button::Keyboard(keyboard::Key::Right)) => current_input = current_input - input::RIGHT,
      Input::Release(Button::Keyboard(keyboard::Key::Space)) => current_input = current_input - input::JUMP,
      Input::Release(Button::Keyboard(keyboard::Key::Z)) => current_input = current_input - input::WHISTLE,
      _ => ()
    };
    self.context.input.1 = current_input;
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
    input: (input::Input::empty(), input::Input::empty()),
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
