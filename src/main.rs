use ggez::{ Context, ContextBuilder, GameResult };
use ggez::graphics::{ self, Color };
use ggez::event::{ self, EventHandler };
use ggez::conf::*;

mod util;
use util::color::CSS3;

fn main() {
    let window_mode = WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: true,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };
    
    let window_setup = WindowSetup {
        title: "rustgame".to_owned(),
        icon: "".to_owned(),
        samples: NumSamples::One,
        vsync: true,
        srgb: true,
    };

    let conf = Conf {
        window_mode: window_mode,
        window_setup: window_setup,
        backend: Backend::default(),
    };

    let (mut ctx, event_loop) = ContextBuilder::new("rustgame", "5Stygian")
        .default_conf(conf)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
} // fn main

struct MyGame {
} // MyGame

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
        } // MyGame
    } // pub fn new
} // impl MyGame

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())
    } // fn update

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        
        let rect = util::Rect(
            ctx, 
            100.0, 100.0, 
            200.0, 300.0,
            CSS3::RED,
        );

        canvas.draw(&rect, graphics::DrawParam::default());
        
        canvas.finish(ctx)
    } // fn draw
} // impl EventHandler for MyGame
