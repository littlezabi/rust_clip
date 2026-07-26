mod backends;
mod models;
mod ui;

use backends::clipboard::start_clipboard_listener;
use std::sync::mpsc;

fn main() -> eframe::Result<()> {
    let (tx, rx) = mpsc::channel();
    start_clipboard_listener(tx);
    ui::app::ClipboardApp::run(rx)
}
