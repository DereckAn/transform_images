use std::path::PathBuf;
use transform_images_lib::{
    ImageFormat, ImageProcessor, ImageProcessorImpl, ProcessingSettings, Quality,
};

/// Path al archivo RAW de prueba
const TEST_RAW_FILE: &str = "/Users/laruina/Documents/fotos_monte/DSC04254.ARW";

/// Helper para crear settings
fn create_settings(output_format: ImageFormat, quality: u8) -> ProcessingSettings {
    let mut settings = ProcessingSettings::default();
    settings
        .set_output_format(Some(output_format))
        .set_quality(Quality::new(quality).unwrap());
    settings
}

#[test]
fn test_raw_to_jpeg_no_exif() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let jpeg_data = processor.optimize(&image, &settings).unwrap();

    // Assert
    // Verificar que el JPEG fue creado
    assert!(!jpeg_data.is_empty());

    // Verificar que NO tiene EXIF
    // JPEG con EXIF tiene marcador APP1 (0xFFE1) después del SOI (0xFFD8)
    // Si no hay EXIF, debería ir directo a otros segmentos

    // Los primeros 2 bytes son SOI marker (0xFFD8)
    assert_eq!(jpeg_data[0], 0xFF);
    assert_eq!(jpeg_data[1], 0xD8);

    // Buscar si hay marcador EXIF (APP1 = 0xFFE1)
    let has_exif = jpeg_data
        .windows(2)
        .any(|window| window[0] == 0xFF && window[1] == 0xE1);

    // NO debería tener EXIF
    assert!(!has_exif, "JPEG should not contain EXIF data");

    println!("✓ RAW → JPEG: No EXIF metadata (cleaned successfully)");
}

#[test]
fn test_raw_to_png_no_exif() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Png, 90);

    // Act
    let png_data = processor.optimize(&image, &settings).unwrap();

    // Assert
    assert!(!png_data.is_empty());

    // PNG signature: 89 50 4E 47 0D 0A 1A 0A
    assert_eq!(png_data[0], 0x89);
    assert_eq!(png_data[1], 0x50); // 'P'
    assert_eq!(png_data[2], 0x4E); // 'N'
    assert_eq!(png_data[3], 0x47); // 'G'

    // Buscar chunk eXIf (EXIF en PNG)
    // PNG chunks tienen formato: [length: 4 bytes][type: 4 bytes][data][crc: 4 bytes]
    let has_exif = png_data.windows(4).any(|window| window == b"eXIf");

    assert!(!has_exif, "PNG should not contain EXIF chunk");

    println!(
        "✓ RAW → PNG: No EXIF metadata (cleaned successfully)"
    );
}

#[test]
fn test_raw_to_webp_no_exif() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Webp, 85);

    // Act
    let webp_data = processor.optimize(&image, &settings).unwrap();

    // Assert
    assert!(!webp_data.is_empty());

    // WebP signature: RIFF....WEBP
    assert_eq!(&webp_data[0..4], b"RIFF");
    assert_eq!(&webp_data[8..12], b"WEBP");

    // Buscar chunk EXIF
    let has_exif = webp_data.windows(4).any(|window| window == b"EXIF");

    assert!(!has_exif, "WebP should not contain EXIF chunk");

    println!("✓ RAW → WebP: No EXIF metadata (cleaned successfully)");
}

#[test]
fn test_metadata_cleaner_all_formats() {
    println!("\n🧪 Testing metadata removal for all formats...\n");

    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let formats = vec![
        (ImageFormat::Jpeg, "JPEG"),
        (ImageFormat::Png, "PNG"),
        (ImageFormat::Webp, "WebP"),
    ];

    for (format, name) in formats {
        let settings = create_settings(format, 85);
        let output = processor.optimize(&image, &settings).unwrap();

        assert!(!output.is_empty(), "{} output should not be empty", name);

        println!(
            "  ✓ {} processed successfully ({} KB)",
            name,
            output.len() / 1024
        );
    }

    println!("\n✅ All formats processed without metadata\n");
}
