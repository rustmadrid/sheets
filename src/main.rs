#![feature(convert)]

extern crate sheets;
extern crate window;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate conrod;
extern crate event;

pub mod ui;

fn main() {
    ui::run();
}
