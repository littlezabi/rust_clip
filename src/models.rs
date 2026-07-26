use std::path::PathBuf;

#[derive(Clone)]
pub enum ClipboardItem {
    Text(String),
    Image { mime: String, data: Vec<u8> },
    Files(Vec<PathBuf>),
}
