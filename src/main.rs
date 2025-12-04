use ggez::{ Context, ContextBuilder, GameResult };
use ggez::graphics::{ self, Color, Rect };
use ggez::event::{ self, EventHandler };

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
        
        let rect = Rect::new(100.0, 100.0, 200.0, 200.0);
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::from_rgb(255, 0, 0), // Red color
        )?;

        canvas.draw(&mesh, graphics::DrawParam::default());
        
        canvas.finish(ctx)
    } // fn draw
} // impl EventHandler for MyGame