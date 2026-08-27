use std::io::BufReader;
use std::path::PathBuf;

use convert_heic_jpeg::convert_to_jpeg;
use image::ImageDecoder;

#[test]
fn converts_real_heic_to_valid_jpeg() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/sample.heic");
    let out_dir = std::env::temp_dir().join("heic2jpg-integration");
    std::fs::create_dir_all(&out_dir).unwrap();
    let output = out_dir.join("sample.jpg");

    convert_to_jpeg(&input, &output, 90).unwrap();

    let decoder = image::codecs::jpeg::JpegDecoder::new(BufReader::new(
        std::fs::File::open(&output).unwrap(),
    ))
    .unwrap();
    assert_eq!(decoder.dimensions(), (1280, 854));

    std::fs::remove_dir_all(&out_dir).unwrap();
}

#[test]
fn rejects_non_heic_file() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    assert!(convert_to_jpeg(&input, std::path::Path::new("unused.jpg"), 90).is_err());
}
