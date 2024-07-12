extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::HashSet;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub struct Player {
    y: f32,
    width: f32,
    height: f32,
}

pub struct Ball {
    x: f32,
    y: f32,
    size: f32,
    direction: [f32; 2],
}

pub struct App {
    gl: GlGraphics,
    player: Player,
    ball: Ball,
    //enemy: Enemy,
    keys: HashSet<Key>,
    dt: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        const REC_WIDTH: f32 = 10.0;
        const REC_HEIGHT: f32 = 50.0;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform;
            //draw border line
            for i in 0..(WINDOW_HEIGHT as i32 / REC_HEIGHT as i32) {
                if i % 2 == 0 {
                    rectangle(
                        WHITE,
                        rectangle::rectangle_by_corners(
                            (WINDOW_WIDTH / 2.0 - REC_WIDTH / 2.0).into(),
                            (i as f32 * REC_HEIGHT).into(),
                            (WINDOW_WIDTH / 2.0 + REC_WIDTH / 2.0).into(),
                            (i as f32 * REC_HEIGHT + REC_HEIGHT).into(),
                        ),
                        transform,
                        gl,
                    );
                }
            }

            //draw ball
            rectangle(
                WHITE,
                rectangle::rectangle_by_corners(
                    self.ball.x.into(),
                    self.ball.y.into(),
                    (self.ball.x + self.ball.size).into(),
                    (self.ball.y + self.ball.size).into(),
                ),
                transform,
                gl,
            );

            //draw player
            rectangle(
                WHITE,
                rectangle::rectangle_by_corners(
                    REC_WIDTH.into(),
                    self.player.y.into(),
                    (REC_WIDTH + self.player.width).into(),
                    (self.player.y + self.player.height).into(),
                ),
                transform,
                gl,
            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.dt = args.dt;
        if self.keys.contains(&Key::A) {
            self.player.y -= 10.0;
        } else if self.keys.contains(&Key::D) {
            self.player.y += 10.0;
        }
        self.player.y = self.player.y.clamp(0.0, WINDOW_HEIGHT - self.player.height);
    }

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => {
                if args.state == ButtonState::Press {
                    self.keys.insert(key);
                } else {
                    self.keys.remove(&key);
                }
            }
            _ => (),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let opengl = OpenGL::V3_1; //3.1

    let mut window: Window = WindowSettings::new(
        "pong",
        [WINDOW_WIDTH.into(), WINDOW_HEIGHT.into()] as [f64; 2],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        player: Player {
            y: 0.0,
            width: 10.0,
            height: 100.0,
        },
        ball: Ball {
            x: WINDOW_WIDTH / 2.0 - 15.0,
            y: WINDOW_HEIGHT / 2.0 - 15.0,
            size: 30.0,
            direction: [-1.0, -1.0],
        },
        keys: HashSet::new(),
        dt: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.input(&args);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
