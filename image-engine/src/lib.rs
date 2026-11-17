use anyhow::Result;
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use std::path::Path;

// ... existing code ...

pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, FilterType::Lanczos3)
}

pub fn resize_preserve_aspect(img: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    img.resize(max_width, max_height, FilterType::Lanczos3)
}

pub fn save_image(
    img: &DynamicImage,
    path: &Path,
    format: ImageFormat,
    quality: Option<u8>,
) -> Result<()> {
    match format {
        ImageFormat::Jpeg => {
            let mut buf = Vec::new();
            let mut encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality.unwrap_or(85));
            encoder.encode_image(img)?;
            std::fs::write(path, buf)?;
        }
        _ => {
            img.save_with_format(path, format)?;
        }
    }
    Ok(())
}

pub fn convert_and_save(
    src: &Path,
    dest: &Path,
    format: ImageFormat,
    quality: Option<u8>,
) -> Result<()> {
    let img = load_image(src)?;
    save_image(&img, dest, format, quality)
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: Option<ImageFormat>,
    pub color_type: image::ColorType,
}

#[derive(Debug)]
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

pub fn load_image(path: &Path) -> Result<DynamicImage> {
    image::open(path).map_err(|e| anyhow::anyhow!("Failed to load image {}: {}", path.display(), e))
}

pub fn get_image_info(path: &Path) -> Result<ImageInfo> {
    let reader = image::ImageReader::open(path)?.with_guessed_format()?;
    let format = reader.format();
    let img = reader.decode()?;
    Ok(ImageInfo {
        width: img.width(),
        height: img.height(),
        format,
        color_type: img.color(),
    })
}

pub fn decode_to_rgba(path: &Path) -> Result<DecodedImage> {
    let img = load_image(path)?;
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    Ok(DecodedImage {
        width,
        height,
        rgba: rgba_img.into_raw(),
    })
}

pub fn rotate_image(img: &DynamicImage, degrees: i32) -> DynamicImage {
    match degrees.rem_euclid(360) {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img.clone(),
    }
}

pub fn supported_extensions() -> &'static [&'static str] {
    &["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "tif"]
}

pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| supported_extensions().contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_extensions_contains_common() {
        let exts = supported_extensions();
        assert!(exts.contains(&"jpg") || exts.contains(&"jpeg"));
        assert!(exts.contains(&"png"));
        assert!(exts.contains(&"webp"));
        assert!(exts.contains(&"gif"));
        assert!(exts.contains(&"bmp"));
    }

    #[test]
    fn test_is_supported_image() {
        assert!(is_supported_image(Path::new("photo.jpg")));
        assert!(is_supported_image(Path::new("image.PNG")));
        assert!(!is_supported_image(Path::new("video.mp4")));
        assert!(!is_supported_image(Path::new("noext")));
    }
}
