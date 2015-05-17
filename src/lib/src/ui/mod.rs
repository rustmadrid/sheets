use ::glutin_window::{GlutinWindow};
use ::conrod;
use ::opengl_graphics::glyph_cache::GlyphCache;
use ::opengl_graphics::{OpenGL, GlGraphics};
use ::sheet::{Coord, Value, Formula};
use ::std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use ::std::sync::Mutex;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const CELL_WIDTH: usize = WINDOW_WIDTH as usize / 5;
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

    fn set<'a, 'b>(&'a mut self, coord: Coord, val: &'b str) {
        self[coord] = match val {
            "" => None,
            x => Some(x.to_string()),
        }
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

struct State {
    editing: Option<Coord>,
    editing_text: String,
}

pub fn run<F>(sheet_select: &mut F, event_stream: SyncSender<UIEvent>)
    where F: Send + FnMut(Coord, Coord) -> Receiver<(Coord, Value)>
{
    use event::*;

    let grid = Mutex::new(CellGrid::new());
    let grid_ref = &grid;
    let (events_sender, events_recv) = sync_channel(0);

    let guard = ::std::thread::scoped(move|| {
        let sheet_selection = sheet_select(Coord(0, 0), Coord(GRID_COLUMNS-1, GRID_ROWS-1));

        let mut running = true;
        while running {
            select! {
                x = events_recv.recv() => {
                    match x {
                        Ok(x) => if let Err(_) = event_stream.send(x) {
                            running = false;
                        },
                        Err(_) => { running = false; },
                    }  
                },
                x = sheet_selection.recv() => {
                    match x {
                        Ok((Coord(col, row), value)) => {
                            let mut g = grid_ref.lock().unwrap();
                            match value {
                                Ok(v) => {
                                    g.set(Coord(col, row), ::parser::format_formula(&Formula::Atom(*v)).as_str());
                                },
                                Err(x) => {
                                    g.set(Coord(col, row), format!("<E:{:?}>", x).as_str())
                                }
                            }
                        },
                        Err(_) => { running = false; },
                    }
                }
            };
        };
    });
        
    let mut state = State{
        editing: None,
        editing_text: "".to_string(),
    };
    let opengl = OpenGL::_3_2;
    let window = make_window(opengl);
    let mut ui = make_ui();
    let mut gl = GlGraphics::new(opengl);

    let event_iter = window.events().ups(180).max_fps(60);
    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                draw_ui(gl, &mut ui, &*grid.lock().unwrap(), &mut state, &events_sender);
            });
        }
    }

    guard.join();
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

fn draw_ui<'a>(gl: &mut GlGraphics, ui: &mut conrod::Ui<GlyphCache<'a>>, grid: &CellGrid, state: &mut State, events: &SyncSender<UIEvent>) {
    use conrod::{Background, Colorable, WidgetMatrix, Button, Labelable, Positionable,
        Sizeable, Widget, TextBox};
    
    Background::new().rgb(1.0, 1.0, 1.0).draw(ui, gl);

    WidgetMatrix::new(GRID_COLUMNS, GRID_ROWS)
    .xy(0.0, 0.0)
    .dimensions(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64)
    .each_widget(ui, |ui, num, col, row, pos, dim| {
        let &mut State{ref mut editing, ref mut editing_text} = state;
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
        let &mut State{ref mut editing, ref mut editing_text} = state;
        TextBox::new(editing_text)
            .middle()
            .width(500.0).height(100.0)
            .react(|s: &mut String| {
                println!("REACT {}", s);
                match ::parser::parse_formula(s.as_str()) {
                    Ok(f) => {
                        events.send(UIEvent::EditCell(coord, Box::new(f))).unwrap();
                        *editing = None;
                    },
                    Err(x) => println!("{}", x),
                }
            })
            .set(TEXTBOX_ID, ui);
    }

    ui.draw(gl);
}

pub enum UIEvent {
    EditCell(Coord, Box<Formula>),
}
