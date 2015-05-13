use ::glutin_window::{GlutinWindow};
use ::conrod;
use ::opengl_graphics::glyph_cache::GlyphCache;
use ::opengl_graphics::{OpenGL, GlGraphics};

pub struct Window<'a> {
    window: GlutinWindow,
    ui: conrod::Ui<GlyphCache<'a>>,
    gl: GlGraphics,
}

impl<'a> Window<'a> {
    pub fn new() -> Self {
        let opengl = OpenGL::_3_2;
        
        Window{
            window: Window::make_window(opengl),
            ui: Window::make_ui(),
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn run(mut self) {
        use event::*;
        use conrod::{Background, Colorable};

        let event_iter = self.window.events().ups(180).max_fps(60);
        for event in event_iter {
            self.ui.handle_event(&event);
            if let Some(args) = event.render_args() {
                let gl = &mut self.gl;
                let ui = &mut self.ui;
                gl.draw(args.viewport(), |_, gl| {
                    // Draw the background.
                    Background::new().rgb(0.2, 0.25, 0.4).draw(ui, gl);

                    // Draw our Ui!
                    ui.draw(gl);
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
                Size{width: 800, height: 600}
            )
        )
    }

    fn make_ui() -> conrod::Ui<GlyphCache<'a>> {
        use std::path::Path;
        use opengl_graphics::glyph_cache::GlyphCache;

        let font_path = Path::new("./assets/NotoSans-Regular.ttf");
        let theme = conrod::Theme::default();
        let glyph_cache = GlyphCache::new(&font_path).unwrap();

        conrod::Ui::new(glyph_cache, theme)
    }
}