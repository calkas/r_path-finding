use piston_window::{Context, G2d, Glyphs, Text, Transformed};
#[allow(dead_code)]
pub mod color {
    pub const ORANGE: [f32; 4] = [255.0, 215.0, 0.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const GREEN: [f32; 4] = [0.0, 255.0, 0.0, 1.0];
    pub const RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];
    pub const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
}

pub fn draw_text(
    text: &str,
    pos: [f64; 2],
    size: u32,
    color: [f32; 4],
    glyph: &mut Glyphs,
    c: &Context,
    g: &mut G2d,
) {
    let transform = c.transform.trans(pos[0], pos[1]);

    for (line_number, line) in text.lines().enumerate() {
        Text::new_color(color, size)
            .draw(
                line,
                glyph,
                &c.draw_state,
                transform.trans(0.0, line_number as f64 * 15.0),
                g,
            )
            .unwrap();
    }
}
