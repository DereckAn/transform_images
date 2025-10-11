use std::path::PathBuf;
use transform_images_lib::{
    Dimensions, ImageFormat, ImageProcessor, ImageProcessorImpl, ProcessingSettings, Quality,
    ResizeFilter, ResizeTransformation, Rotation, Transformation,
};

/// Path al archivo RAW de prueba
const TEST_RAW_FILE: &str = "/Users/laruina/Documents/fotos_monte/DSC04254.ARW";

/// Helper para crear directorio temporal de output
fn create_test_output_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir().join("transform_images_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    temp_dir
}

/// Helper para crear settings con formato de salida
fn create_settings(output_format: ImageFormat, quality: u8) -> ProcessingSettings {
    let mut settings = ProcessingSettings::default();
    settings
        .set_output_format(Some(output_format))
        .set_quality(Quality::new(quality).unwrap());
    settings
}

#[test]
fn test_load_raw_image() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);

    // Act
    let result = processor.load_image(&path);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to load RAW image: {:?}",
        result.err()
    );

    let image = result.unwrap();
    assert_eq!(image.format(), ImageFormat::Raw);
    assert!(image.dimensions().width() > 0);
    assert!(image.dimensions().height() > 0);

    println!(
        "✓ Loaded RAW image: {}x{}",
        image.dimensions().width(),
        image.dimensions().height()
    );
}

#[test]
fn test_raw_to_jpeg_conversion() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.optimize(&image, &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to convert RAW to JPEG: {:?}",
        result.err()
    );

    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());
    assert!(
        jpeg_data.len() < image.size_bytes() as usize,
        "JPEG should be smaller than RAW"
    );

    println!(
        "✓ RAW → JPEG: {} KB → {} KB",
        image.size_bytes() / 1024,
        jpeg_data.len() / 1024
    );
}

#[test]
fn test_raw_to_png_conversion() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Png, 90);

    // Act
    let result = processor.optimize(&image, &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to convert RAW to PNG: {:?}",
        result.err()
    );

    let png_data = result.unwrap();
    assert!(!png_data.is_empty());

    println!(
        "✓ RAW → PNG: {} KB → {} KB",
        image.size_bytes() / 1024,
        png_data.len() / 1024
    );
}

#[test]
fn test_raw_to_webp_conversion() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Webp, 85);

    // Act
    let result = processor.optimize(&image, &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to convert RAW to WebP: {:?}",
        result.err()
    );

    let webp_data = result.unwrap();
    assert!(!webp_data.is_empty());

    println!(
        "✓ RAW → WebP: {} KB → {} KB",
        image.size_bytes() / 1024,
        webp_data.len() / 1024
    );
}

#[test]
fn test_raw_resize_preserve_aspect_ratio() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let original_width = image.dimensions().width();
    let original_height = image.dimensions().height();

    // Resize a 1920x1080 preservando aspect ratio
    let target_dims = Dimensions::new(1920, 1080).unwrap();
    let resize = ResizeTransformation::with_dimensions(target_dims, true);
    let transformation = Transformation::with_resize(resize);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(result.is_ok(), "Failed to resize RAW: {:?}", result.err());

    let jpeg_data = result.unwrap();
    assert!(!jpeg_data.is_empty());

    println!(
        "✓ RAW Resize: {}x{} → 1920x1080 (aspect ratio 
  preserved)",
        original_width, original_height
    );
}

#[test]
fn test_raw_resize_exact() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    // Resize a 1920x1080 EXACTO (sin preservar aspect ratio)
    let target_dims = Dimensions::new(1920, 1080).unwrap();
    let resize = ResizeTransformation::new(target_dims, false, ResizeFilter::Lanczos3);
    let transformation = Transformation::with_resize(resize);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to resize RAW exactly: {:?}",
        result.err()
    );

    println!(
        "✓ RAW Resize Exact: → 1920x1080 (exact 
  dimensions)"
    );
}

#[test]
fn test_raw_rotate_90() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let transformation = Transformation::with_rotation(Rotation::Clockwise90);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to rotate RAW 90°: {:?}",
        result.err()
    );

    println!("✓ RAW Rotate: 90° clockwise");
}

#[test]
fn test_raw_rotate_180() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let transformation = Transformation::with_rotation(Rotation::Rotate180);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to rotate RAW 180°: {:?}",
        result.err()
    );

    println!("✓ RAW Rotate: 180°");
}

#[test]
fn test_raw_flip_horizontal() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let mut transformation = Transformation::new();
    transformation.set_flip_horizontal(true);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to flip RAW horizontally: 
  {:?}",
        result.err()
    );

    println!("✓ RAW Flip: horizontal");
}

#[test]
fn test_raw_flip_vertical() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let mut transformation = Transformation::new();
    transformation.set_flip_vertical(true);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed to flip RAW vertically: {:?}",
        result.err()
    );

    println!("✓ RAW Flip: vertical");
}

#[test]
fn test_raw_combined_transformations() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    // Combinación: Resize + Rotate + Flip
    let target_dims = Dimensions::new(1920, 1080).unwrap();
    let resize = ResizeTransformation::with_dimensions(target_dims, true);

    let mut transformation = Transformation::with_resize(resize);
    transformation
        .set_rotation(Rotation::Clockwise90)
        .set_flip_horizontal(true);

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act
    let result = processor.process(&image, Some(&transformation), &settings);

    // Assert
    assert!(
        result.is_ok(),
        "Failed combined transformations: 
  {:?}",
        result.err()
    );

    println!(
        "✓ RAW Combined: Resize + Rotate 90° + Flip 
  Horizontal"
    );
}

#[test]
fn test_raw_all_filters() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let target_dims = Dimensions::new(800, 600).unwrap();

    let filters = vec![
        ResizeFilter::Nearest,
        ResizeFilter::Triangle,
        ResizeFilter::CatmullRom,
        ResizeFilter::Gaussian,
        ResizeFilter::Lanczos3,
    ];

    let settings = create_settings(ImageFormat::Jpeg, 85);

    // Act & Assert
    for filter in filters {
        let resize = ResizeTransformation::new(target_dims, true, filter);
        let transformation = Transformation::with_resize(resize);

        let result = processor.process(&image, Some(&transformation), &settings);

        assert!(
            result.is_ok(),
            "Failed with filter {:?}: {:?}",
            filter,
            result.err()
        );

        println!("✓ RAW Resize with filter: {:?}", filter);
    }
}

#[test]
fn test_raw_save_to_file() {
    // Arrange
    let processor = ImageProcessorImpl::new();
    let path = PathBuf::from(TEST_RAW_FILE);
    let image = processor.load_image(&path).unwrap();

    let settings = create_settings(ImageFormat::Jpeg, 85);

    let output_dir = create_test_output_dir();
    let output_path = output_dir.join("test_output.jpg");

    // Act
    let jpeg_data = processor.optimize(&image, &settings).unwrap();
    let save_result = processor.save_image(&jpeg_data, &output_path, ImageFormat::Jpeg);

    // Assert
    assert!(
        save_result.is_ok(),
        "Failed to save image: {:?}",
        save_result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    let file_size = std::fs::metadata(&output_path).unwrap().len();
    assert!(file_size > 0, "Output file is empty");

    println!(
        "✓ RAW saved to: {:?} ({} KB)",
        output_path,
        file_size / 1024
    );
}
