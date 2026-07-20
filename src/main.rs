use std::path::PathBuf;
use wayland_clipboard_listener::{WlClipboardPasteStream, WlListenType};

mod handlers;
mod models;

fn main() {
    println!("Starting event-driven Wayland Clipboard Listener (Text, Images, Files)...\n");

    let mut stream = match WlClipboardPasteStream::init(WlListenType::ListenOnCopy) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize Wayland clipboard listener: {e}");
            return;
        }
    };

    println!("Listening for clipboard events (Text, Image, Files)...\n");
    for context_res in stream.paste_stream() {
        match context_res {
            Ok(msg) => {
                let mime = &msg.context.mime_type;
                let data = &msg.context.context;

                if mime.starts_with("image/") {
                    handlers::handle_image::process_image(mime, data);
                } else if mime.contains("uri-list") {
                    let text = String::from_utf8_lossy(data);
                    let paths: Vec<PathBuf> = text
                        .lines()
                        .filter_map(|l| l.strip_prefix("file://"))
                        .map(PathBuf::from)
                        .collect();
                    println!("Received file list ({} files): {paths:#?}", paths.len());
                } else if mime.starts_with("text/") || mime == "UTF8_STRING" || mime == "STRING" {
                    let text = String::from_utf8_lossy(data);
                    if !text.trim().is_empty() {
                        handlers::handle_text::process_text(&text);
                    }
                } else {
                    println!("Received format `{mime}` ({} bytes)", data.len());
                }
            }
            Err(e) => {
                eprintln!("Wayland clipboard error: {e}");
            }
        }
    }
}