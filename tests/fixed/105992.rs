#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

const WHITE: Color = Color(0xCF, 0xCE, 0xC1);
const YELLOW: Color = Color(0xFF, 0xFF, 0x00);

struct Tile {
    glyph: char,
    color: Color,
}

struct BufferRenderer {
    rows: i32,
    cols: i32,
    buffer: Vec<Tile>,
}

impl BufferRenderer {
    fn render(&mut self, col: i32, row: i32, glyph: char, color: Color) {
        let idx = (row * self.cols + col) as usize;
        let tile = &mut self.buffer[idx];
        tile.glyph = glyph;
        tile.color = color;
    }
}

fn render_some_lines(
    renderer: &mut BufferRenderer,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
) {
    renderer.render(x, y, '╭', color);
    renderer.render(x + width, y, '╮', color);
    renderer.render(x, y + height, '╰', color);
    renderer.render(x + width, y + height, '╯', color);
    for xi in x + 1..x + width {
        renderer.render(xi, y, '─', color);
        renderer.render(xi, y + height, '─', color);
    }
}

trait State {
    fn render(&self, renderer: &mut BufferRenderer);
}

struct App {}

impl State for App {
    fn render(&self, renderer: &mut BufferRenderer) {
        render_some_lines(renderer, 0, 0, renderer.cols - 1, renderer.rows - 1, WHITE);
        render_some_lines(renderer, 0, 0, 1, 1, YELLOW);
    }
}
