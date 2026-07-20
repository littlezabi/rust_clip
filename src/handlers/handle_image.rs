use std::path::Path;

pub fn process_image(mime_type: &str, bytes: &[u8]) {
    println!(
        "Received image [{}] (Size: {} bytes / {:.2} KB)",
        mime_type,
        bytes.len(),
        bytes.len() as f64 / 1024.0
    );
}

pub fn process_png_bytes(bytes: &[u8], path: Option<&Path>) {
    println!("Received PNG image (Size: {} bytes / {:.2} KB)", bytes.len(), bytes.len() as f64 / 1024.0);
    if let Some(p) = path {
        println!("Image File Path: {}", p.display());
    } else {
        println!("Image Source: Clipboard (In-Memory)");
    }
}