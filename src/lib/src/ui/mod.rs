use ::glutin_window::{GlutinWindow};
use ::conrod;
use ::opengl_graphics::glyph_cache::GlyphCache;
use ::opengl_graphics::{OpenGL, GlGraphics};
use ::sheet::Coord;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const CELL_WIDTH: usize = WINDOW_WIDTH as usize / 10;
const CELL_HEIGHT: usize = WINDOW_HEIGHT as usize / 20;
const GRID_COLUMNS: usize = WINDOW_WIDTH as usize / CELL_WIDTH - 1;
const GRID_ROWS: usize = WINDOW_HEIGHT as usize / CELL_HEIGHT - 1;
const NUM_CELLS: usize = GRID_COLUMNS * GRID_ROWS;
const TEXTBOX_ID: usize = NUM_CELLS + 1;

struct CellGrid(Vec<Option<String>>);

impl CellGrid {
    fn new() -> Self {
        let mut m = Vec::with_capacity(NUM_CELLS);  
        for _ in 0 .. m.capacity() {
            m.push(None);
        }
        CellGrid(m)
    }

    fn get_str(&self, col: usize, row: usize) -> &str {
        match self[Coord(col, row)] {
            Some(ref x) => x.as_str(),
            None => ""
        }
    }

    fn set<'a, 'b>(&mut self, coord: Coord, val: &'b str) {
        self[coord] = match val {
            "" => None,
            x => Some(x.to_string()),
        };
    }
}

impl ::std::ops::Index<Coord> for CellGrid {
    type Output = Option<String>;

    fn index<'a>(&'a self, Coord(col, row): Coord) -> &'a Self::Output {
        let &CellGrid(ref m) = self;
        &m[col * GRID_ROWS + row]
    }
}

impl ::std::ops::IndexMut<Coord> for CellGrid {
    fn index_mut<'a>(&'a mut self, Coord(col, row): Coord) -> &'a mut Self::Output {
        let &mut CellGrid(ref mut m) = self;
        &mut m[col * GRID_ROWS + row]
    }
}

pub struct UI {
    grid: CellGrid,
    editing: Option<Coord>,
    editing_text: String,
}

pub fn run() {
    use event::*;
    let opengl = OpenGL::_3_2;
    let window = make_window(opengl);
    let mut ui = make_ui();
    let mut gl = GlGraphics::new(opengl);

    let mut state = UI {
        grid: CellGrid::new(),
        editing: None,
        editing_text: "".to_string(),
    };

    let event_iter = window.events().ups(180).max_fps(60);
    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                draw_ui(gl, &mut ui, &mut state);
            });
        }
    }
}

fn make_window(opengl: OpenGL) -> GlutinWindow {
    use glutin_window::GlutinWindow;
    use window::{WindowSettings, Size};

    GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Sheets".to_string(),
            Size{width: WINDOW_WIDTH, height: WINDOW_HEIGHT}
        )
    )
}

fn make_ui<'a>() -> conrod::Ui<GlyphCache<'a>> {
    use std::path::Path;
    use opengl_graphics::glyph_cache::GlyphCache;

    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = conrod::Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();

    conrod::Ui::new(glyph_cache, theme)
}

fn draw_ui<'a>(gl: &mut GlGraphics, ui: &mut conrod::Ui<GlyphCache<'a>>, state: &mut UI) {
    use conrod::{Background, Colorable, WidgetMatrix, Button, Labelable, Positionable,
        Sizeable, Widget, TextBox};
    
    Background::new().rgb(1.0, 1.0, 1.0).draw(ui, gl);

    WidgetMatrix::new(GRID_COLUMNS, GRID_ROWS)
    .xy(0.0, 0.0)
    .dimensions(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64)
    .each_widget(ui, |ui, num, col, row, pos, dim| {
        let &mut UI{ref grid, ref mut editing, ref mut editing_text} = state;
        Button::new()
            .label(grid.get_str(col, row))
            .point(pos)
            .dim(dim)
            .react(|| {
                *editing = Some(Coord(col, row));
                *editing_text = grid.get_str(col, row).to_string();
            })
            .enabled(if let Some(_) = state.editing { false } else { true })
            .set(num, ui);
    });

    if let Some(coord) = state.editing {
        let &mut UI{ref mut grid, ref mut editing, ref mut editing_text} = state;
        TextBox::new(editing_text)
            .middle()
            .width(500.0).height(100.0)
            .react(|s: &mut String| {
                grid.set(coord, s.as_str());
                *editing = None;
            })
            .set(TEXTBOX_ID, ui);
    }

    ui.draw(gl);
}
