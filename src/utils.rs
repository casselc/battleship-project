use piston_window::ellipse;
use piston_window::line;
use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = (x as f64) * 25.0;
    let gui_y = (y as f64) * 25.0;
    rectangle(color, [gui_x, gui_y, 25.0, 25.0], con.transform, g);
    line(
        [0.0, 0.0, 0.0, 1.0],
        1.0,
        [gui_x, gui_y, gui_x + 25.0, gui_y],
        con.transform,
        g,
    );
    line(
        [0.0, 0.0, 0.0, 1.0],
        1.0,
        [gui_x, gui_y, gui_x, gui_y + 25.0],
        con.transform,
        g,
    );
    line(
        [0.0, 0.0, 0.0, 1.0],
        1.0,
        [gui_x + 25.0, gui_y, gui_x + 25.0, gui_y + 25.0],
        con.transform,
        g,
    );
    line(
        [0.0, 0.0, 0.0, 1.0],
        1.0,
        [gui_x, gui_y + 25.0, gui_x + 25.0, gui_y + 25.0],
        con.transform,
        g,
    );
}

pub fn draw_circle(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = (x as f64 + 0.15) * 25.0;
    let gui_y = (y as f64 + 0.15) * 25.0;
    ellipse(
        color,
        [gui_x, gui_y, 25.0 * 0.7, 25.0 * 0.7],
        con.transform,
        g,
    );
}
