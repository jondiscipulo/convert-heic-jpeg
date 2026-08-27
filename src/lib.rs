use std::io::BufWriter;
use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::{ExtendedColorType, ImageEncoder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("failed to decode HEIC image: {0}")]
    Decode(#[from] heif_oxide::HeifError),
    #[error("failed to create output file: {0}")]
    CreateOutput(#[source] std::io::Error),
    #[error("failed to encode JPEG: {0}")]
    Encode(#[source] image::ImageError),
}

/// Converts a single HEIC/HEIF image to a JPEG file.
pub fn convert_to_jpeg(input: &Path, output: &Path, quality: u8) -> Result<(), ConvertError> {
    let decoded = heif_oxide::decode_file(input)?;
    let rgb = rgba_to_rgb(&decoded.to_rgba8());

    let file = std::fs::File::create(output).map_err(ConvertError::CreateOutput)?;
    let mut writer = BufWriter::new(file);
    JpegEncoder::new_with_quality(&mut writer, quality)
        .write_image(&rgb, decoded.width, decoded.height, ExtendedColorType::Rgb8)
        .map_err(ConvertError::Encode)?;
    Ok(())
}

/// Drops the alpha channel, producing interleaved RGB bytes for JPEG output.
fn rgba_to_rgb(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4)
        .flat_map(|px| [px[0], px[1], px[2]])
        .collect()
}

/// Derives the default output path for a HEIC input file.
pub fn default_output_path(input: &Path, output_dir: Option<&Path>) -> PathBuf {
    match output_dir {
        Some(dir) => dir.join(output_file_name(input)),
        None => input.with_extension("jpg"),
    }
}

fn output_file_name(input: &Path) -> String {
    let stem = input
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "output".to_owned());
    format!("{stem}.jpg")
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageDecoder;
    use image::codecs::jpeg::JpegDecoder;
    use std::io::Cursor;

    #[test]
    fn flattens_alpha_channel() {
        let rgba = [255, 0, 0, 255, 0, 255, 0, 0, 0, 0, 255, 128];
        let rgb = rgba_to_rgb(&rgba);
        assert_eq!(rgb, vec![255, 0, 0, 0, 255, 0, 0, 0, 255]);
    }

    #[test]
    fn derives_output_path_next_to_input() {
        let input = Path::new("/photos/vacation.HEIC");
        assert_eq!(
            default_output_path(input, None),
            PathBuf::from("/photos/vacation.jpg")
        );
    }

    #[test]
    fn derives_output_path_in_output_dir() {
        let input = Path::new("/photos/vacation.heic");
        assert_eq!(
            default_output_path(input, Some(Path::new("/out"))),
            PathBuf::from("/out/vacation.jpg")
        );
    }

    #[test]
    fn encodes_valid_jpeg() {
        let (width, height) = (16u32, 8u32);
        let rgb: Vec<u8> = (0..width * height)
            .flat_map(|i| {
                let r = (i % 256) as u8;
                let g = (i / 256 % 256) as u8;
                let b = (i / 65536 % 256) as u8;
                [r, g, b]
            })
            .collect();

        let mut out = Vec::new();
        {
            let mut cursor = Cursor::new(&mut out);
            JpegEncoder::new_with_quality(&mut cursor, 90)
                .write_image(&rgb, width, height, ExtendedColorType::Rgb8)
                .unwrap();
        }

        let decoder = JpegDecoder::new(Cursor::new(&out)).unwrap();
        assert_eq!(decoder.dimensions(), (width, height));
        assert!(out.starts_with(&[0xFF, 0xD8]), "missing JPEG SOI marker");
        assert!(out.ends_with(&[0xFF, 0xD9]), "missing JPEG EOI marker");
    }
}
