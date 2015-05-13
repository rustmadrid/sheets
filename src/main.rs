extern crate sheets;
extern crate window;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate conrod;
extern crate event;

fn main() {
    use glutin_window::GlutinWindow;
    use opengl_graphics::{GlGraphics, OpenGL};
    use opengl_graphics::glyph_cache::GlyphCache;
    use window::{WindowSettings, Size};
    use event::*;
    use conrod::{Background, Colorable, Theme, Ui, Widget};
    use std::path::Path;

    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Hello Conrod".to_string(),
            Size{width: 800, height: 600}
        )
    );

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("./assets/NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                // Draw the background.
                Background::new().rgb(0.2, 0.25, 0.4).draw(ui, gl);

                // Draw our Ui!
                ui.draw(gl);

            });
        }
    }
}
