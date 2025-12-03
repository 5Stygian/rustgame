use macroquad::prelude::*;

mod gui;

#[macroquad::main("game")]
pub async fn main() {
    'mainloop: loop {
        clear_background(gui::GRAY);

        next_frame().await
    } // mainloop
} // main