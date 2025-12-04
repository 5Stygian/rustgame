use ggez::{ Context };
use ggez::graphics::{ self, Color, Rect, Mesh };

#[allow(non_snake_case)]
/// This is a helper function to make creating Rects more intuitive.
pub fn Rect(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) -> Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(x, y, w, h),
        color,
    ).unwrap()
} // pub rn Rect
