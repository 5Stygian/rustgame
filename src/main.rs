use ggez::{ Context, ContextBuilder, GameResult };
use ggez::graphics::{ self, Color };
use ggez::event::{ self, EventHandler };

mod util;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "5Stygian")
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
    } // fn new
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
            Color::from_rgb(255, 0, 0)
        );

        canvas.draw(&rect, graphics::DrawParam::default());
        
        canvas.finish(ctx)
    } // fn draw
} // impl EventHandler for MyGame