use macroquad::prelude::*;

use gui::colors;

#[macroquad::main("game")]
pub async fn main() {
    'mainloop: loop {
        clear_background(gui::colors::rgba(0.8, 0.24, 0.1, 1.0));

        next_frame().await
    } // mainloop
} // main