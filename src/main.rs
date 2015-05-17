#![feature(scoped)]

extern crate sheets_lib;

use ::std::sync::mpsc::sync_channel;
use ::std::sync::Mutex;

fn main() {
    let sheet = Mutex::new(sheets_lib::sheet::Sheet::new());
    let sheet_ref = &sheet;

    let (event_send, event_recv) = sync_channel(0);
    
    let guard = ::std::thread::scoped(move|| {
        use sheets_lib::ui::UIEvent::{EditCell};
        let mut running = true;
        while running {
            match event_recv.recv() {
                Ok(EditCell(coord, formula)) => {
                    sheet_ref.lock().unwrap().set(coord, *formula);
                },
                Err(_) => { running = false },
            };
        }
    });

    sheets_lib::ui::run(&mut |from, to| {
        sheet.lock().unwrap().select(from, to)
    }, event_send);

    guard.join();
}
