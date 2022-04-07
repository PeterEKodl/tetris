use rustbox::*;

pub trait Render {
    fn render(&self, rustbox: &RustBox);
}

// Draw a box at the specified position.
pub fn render_box(
    corner_x: usize,
    corner_y: usize,
    width: usize,
    height: usize,
    color: Color,
    rustbox: &RustBox,
) {
    for x in corner_x + 1..(corner_x + width - 1) {
        rustbox.print_char(x, corner_y, RB_NORMAL, color, Color::Default, '-');
        rustbox.print_char(
            x,
            corner_y + height - 1,
            RB_NORMAL,
            color,
            Color::Default,
            '-',
        );
    }

    for y in corner_y + 1..(corner_y + height - 1) {
        rustbox.print_char(corner_x, y, RB_NORMAL, color, Color::Default, '|');
        rustbox.print_char(
            corner_x + width - 1,
            y,
            RB_NORMAL,
            color,
            Color::Default,
            '|',
        );
    }

    rustbox.print_char(corner_x, corner_y, RB_NORMAL, color, Color::Default, '+');
    rustbox.print_char(
        corner_x + width - 1,
        corner_y,
        RB_NORMAL,
        color,
        Color::Default,
        '+',
    );
    rustbox.print_char(
        corner_x,
        corner_y + height - 1,
        RB_NORMAL,
        color,
        Color::Default,
        '+',
    );
    rustbox.print_char(
        corner_x + width - 1,
        corner_y + height - 1,
        RB_NORMAL,
        color,
        Color::Default,
        '+',
    );
}

pub fn render_filled_box(
    corner_x: usize,
    corner_y: usize,
    width: usize,
    height: usize,
    color: Color,
    ch: char,
    rustbox: &RustBox,
) {
    for y in corner_y..(corner_y + height) {
        for x in corner_x..(corner_x + width) {
            rustbox.print_char(x, y, RB_NORMAL, color, Color::Default, ch);
        }
    }
}
