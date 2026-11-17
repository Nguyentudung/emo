use media_core::MediaFile;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_directory(path: &Path) -> anyhow::Result<Vec<MediaFile>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_path_buf();
        if path.is_file() {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            let is_image = matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp"
            );
            let is_video = matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov" | "webm");
            if is_image || is_video {
                files.push(MediaFile {
                    path,
                    name,
                    size,
                    is_image,
                    is_video,
                });
            }
        }
    }
    Ok(files)
}
