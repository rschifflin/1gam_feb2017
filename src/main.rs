#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate cpal;
#[macro_use] extern crate conrod;
#[macro_use] extern crate bitflags;
extern crate float;
extern crate futures;
extern crate glium;
extern crate glium_graphics;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate graphics;
extern crate piston;
extern crate specs;
extern crate collider;
extern crate itertools;
extern crate rand;

mod components;
mod systems;
mod geom;
mod events;
#[allow(dead_code)] mod colors;
/*
mod sound;
*/
mod world;
mod input;
mod map;
mod progress;
mod facing;
mod fsm;
mod ui;


use piston::window::WindowSettings;
use piston::input::*;
use glium_graphics::{Glium2d, GliumGraphics, OpenGL, GliumWindow};
use glium_graphics::Texture;
use glium::{Surface, Frame};
use specs::{/*RunArg,*/ Join, World};
use itertools::Itertools;
//use futures::sync::mpsc::channel;
//use sound::{SoundEvent, spawn_audio_thread};

pub struct App {
  gl: Glium2d, // OpenGL drawing backend.
  context: world::Context,
  planner: specs::Planner<world::Context>,
  time_since_update: f64
}

impl App {
  fn render(&mut self, args: &RenderArgs, window: &mut glium_graphics::GliumWindow, texture: &Texture) {
    let world = self.planner.mut_world();
    let mut frame = window.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    self.gl.draw(&mut frame, args.viewport(), |c, g| {
      Self::render_gfx(c, g, texture, world);
    });

    frame.finish().unwrap();
  }

  fn render_gfx(c: graphics::Context, g: &mut GliumGraphics<Frame>, texture: &Texture, world: &mut World) {
    use graphics::{rectangle, Transformed};
    use graphics::types::{Rectangle, SourceRectangle};
    let positions = world.read::<components::Position>();
    let sprites = world.read::<components::Sprite>();
    let collision = world.read::<components::Collision>();
    let cameras = world.read::<components::Camera>();
    let checkpoints = world.read::<components::Checkpoint>();
    let blast_zones = world.read::<components::BlastZone>();
    let game_state = world.read::<components::GameState>();
    for camera in (&cameras).iter() {
      let screen = c.viewport.unwrap().window_size;
      let screen_ar = screen[0] as f64 / screen[1] as f64;
      let letterbox_height = ((camera.screen.w / screen_ar) - camera.screen.h) / 2.0;

      let scale = screen[0] as f64 / camera.screen.w;
      let scaled_h = camera.screen.h * scale;

      let letterboxes = if scaled_h < screen[1] as f64 {
        Some(
          [[0.0, -letterbox_height, camera.screen.w, letterbox_height],
          [0.0, camera.screen.h, camera.screen.w, letterbox_height]]
        )
      } else { None };

      let xform = c.transform
        .scale(scale, scale)
        .trans(0.0, letterboxes.map(|_| letterbox_height).unwrap_or(0.0));
      let draw_state = graphics::DrawState::default();
      let gameplay_area = camera.gameplay_area();
      let ui_area = camera.ui_area();

      let mut images: Vec<(Rectangle, SourceRectangle)> = (&positions, &sprites).iter().filter_map(|(pos, sprite)| {
        sprite
          .as_image_rects(pos.x - gameplay_area.x, pos.y - gameplay_area.y)
          .map(|(rect, source_rect)| (rect, source_rect, sprite.layer as usize))
      }).sorted_by(|&(_, _, l1), &(_, _, ref l2)| l1.cmp(l2))
        .iter()
        .map(|&(rect, src, _)| (rect, src))
        .collect();
      graphics::image::draw_many(&images, colors::WHITE, texture, &draw_state, xform, g);

      rectangle(colors::BLACK, [ui_area.x, ui_area.y, ui_area.w, ui_area.h], xform, g);
      game_state.iter().next().map(|state| {
        graphics::image::draw_many(&ui::draw(state.progress, &ui_area), colors::WHITE, texture, &draw_state, xform, g);
      });

      letterboxes.map(|boxes| {
        rectangle(colors::BLACK, boxes[0], xform, g);
        rectangle(colors::BLACK, boxes[1], xform, g);
      });
    }
  }

  fn input(&mut self, args: &Input) {
    let mut input = &mut self.context.input;
    match *args {
      /*
      Input::Press(x) => println!("Pressed {:?}", x),
      Input::Release(x) => println!("Released {:?}", x),
      */
      Input::Press(Button::Keyboard(keyboard::Key::Up)) => input.on(input::UP),
      Input::Press(Button::Keyboard(keyboard::Key::Down)) => input.on(input::DOWN),
      Input::Press(Button::Keyboard(keyboard::Key::Left)) => input.on(input::LEFT),
      Input::Press(Button::Keyboard(keyboard::Key::Right)) => input.on(input::RIGHT),
      Input::Press(Button::Keyboard(keyboard::Key::Space)) => input.on(input::JUMP),
      Input::Press(Button::Keyboard(keyboard::Key::Z)) => input.on(input::WHISTLE),

      Input::Release(Button::Keyboard(keyboard::Key::Up)) => input.off(input::UP),
      Input::Release(Button::Keyboard(keyboard::Key::Down)) => input.off(input::DOWN),
      Input::Release(Button::Keyboard(keyboard::Key::Left)) => input.off(input::LEFT),
      Input::Release(Button::Keyboard(keyboard::Key::Right)) => input.off(input::RIGHT),
      Input::Release(Button::Keyboard(keyboard::Key::Space)) => input.off(input::JUMP),
      Input::Release(Button::Keyboard(keyboard::Key::Z)) => input.off(input::WHISTLE),
      _ => ()
    };
  }

  fn update(&mut self, &UpdateArgs { dt }: &UpdateArgs) {
    self.time_since_update += dt;
    if self.time_since_update > 0.0166666 {
      self.planner.dispatch(self.context.clone());
      self.context.input.update();
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
    [852, 480]
  )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new game and run it.
  let mut world = specs::World::new();
  world::register(&mut world);

  let mut planner = specs::Planner::<world::Context>::new(world, 4);
  systems::plan_system(&mut planner, systems::director::Director, 0);
  systems::plan_system(&mut planner, systems::behavior::Hero, 10);
  systems::plan_system(&mut planner, systems::behavior::Bird, 11);
  systems::plan_system(&mut planner, systems::behavior::Enemy, 12);
  systems::plan_system(&mut planner, systems::physics::Physics, 20);
  systems::plan_system(&mut planner, systems::camera::Camera, 30);
  systems::plan_system(&mut planner, systems::sprite::Sprite, 31);
  systems::plan_system(&mut planner, systems::song::Song, 32);

  let context = world::Context {
    input: input::InputBuffer::new(),
    //sound_tx: sound_tx
  };

  let mut app = App {
    gl: Glium2d::new(opengl, &window),
    context: context,
    planner: planner,
    time_since_update: 0.0
  };

  let texture: Texture = glium_graphics::Texture::from_path(&mut window, "./assets/bird_and_blob.png", glium_graphics::Flip::None, &glium_graphics::TextureSettings::new()).unwrap();
  while let Some(e) = window.next() {
    if let &Event::Input(ref i) = &e {
      app.input(i);
    }
    e.update(|args| app.update(&args));
    e.render(|args| {
      app.render(&args, &mut window, &texture);
    });
  }
}
