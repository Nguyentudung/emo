use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_image: bool,
    pub is_video: bool,
}

#[derive(Debug, Clone)]
pub enum MediaType {
    Image,
    Video,
    Unknown,
}

pub trait MediaLoader {
    fn load(&self, path: &std::path::Path) -> anyhow::Result<MediaFile>;
}
