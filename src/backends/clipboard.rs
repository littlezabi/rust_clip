use crate::models::ClipboardItem;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use wayland_clipboard_listener::{WlClipboardPasteStream, WlListenType};

pub fn start_clipboard_listener(tx: Sender<ClipboardItem>) {
    std::thread::spawn(move || {
        println!("Background clipboard listener thread started...");

        let mut stream = match WlClipboardPasteStream::init(WlListenType::ListenOnCopy) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to initialize Wayland clipboard listener: {e}");
                return;
            }
        };

        for context_res in stream.paste_stream() {
            match context_res {
                Ok(msg) => {
                    let mime = &msg.context.mime_type;
                    let data = &msg.context.context;

                    let item = if mime.starts_with("image/") {
                        Some(ClipboardItem::Image {
                            mime: mime.clone(),
                            data: data.clone(),
                        })
                    } else if mime.contains("uri-list") {
                        let text = String::from_utf8_lossy(data);
                        let paths: Vec<PathBuf> = text
                            .lines()
                            .filter_map(|l| l.strip_prefix("file://"))
                            .map(PathBuf::from)
                            .collect();
                        Some(ClipboardItem::Files(paths))
                    } else if mime.starts_with("text/") || mime == "UTF8_STRING" || mime == "STRING"
                    {
                        let text = String::from_utf8_lossy(data).to_string();
                        if !text.trim().is_empty() {
                            Some(ClipboardItem::Text(text))
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(clipboard_item) = item {
                        // Send item over channel to the GUI thread
                        let _ = tx.send(clipboard_item);
                    }
                }
                Err(e) => {
                    eprintln!("Wayland clipboard error: {e}");
                }
            }
        }
    });
}
