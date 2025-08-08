use yew_project::image_processing::{generate_gem_art, generate_text_image};
use yew_project::utils::to_excel_column;
use yew_project::models::{ImageFitOption, GemCount, Color};
use std::time::Instant;
use base64::Engine;
use image::{DynamicImage, Rgba, GenericImage};
use std::io::Cursor;
use base64::engine::general_purpose;

#[test]
fn test_to_excel_column() {
    assert_eq!(to_excel_column(1), "A");
    assert_eq!(to_excel_column(26), "Z");
    assert_eq!(to_excel_column(27), "AA");
    assert_eq!(to_excel_column(702), "ZZ");
    assert_eq!(to_excel_column(703), "AAA");
}

#[test]
fn test_generate_gem_art_performance_and_correctness() {
    let img_path = "C:\\Users\\marin\\Documents\\Rust\\gem_art_creator\\test_images\\test_source.JPG";
    let img = image::open(img_path).expect("Failed to open image");
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    let image_data_url = format!("data:image/png;base64,{}", encoded_data);

    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FFFFFF".to_string(), r: 255, g: 255, b: 255, value: "#FFFFFF".to_string() },
        Color { floss_number: "2".to_string(), hex: "000000".to_string(), r: 0, g: 0, b: 0, value: "#000000".to_string() },
    ];
    let margin_mm = 10.0;
    let fit_option = ImageFitOption::Fit;
    let custom_width_mm = Some(210.0);
    let custom_height_mm = Some(297.0);

    let start_time = Instant::now();
    let (gem_image, gem_counts) = generate_gem_art(
        &image_data_url,
        &colors,
        margin_mm,
        &fit_option,
        custom_width_mm,
        custom_height_mm,
    ).unwrap();
    let duration = start_time.elapsed();

    println!("generate_gem_art took: {:?}", duration);

    assert!(!gem_image.is_empty(), "Generated gem image should not be empty");
    assert!(!gem_counts.is_empty(), "Generated gem counts should not be empty");

    // Further assertions can be added here to check the content of gem_image or gem_counts
    // For example, checking dimensions of the image, or specific gem counts if a known input is used.
}

#[test]
fn test_generate_text_image() {
    let gem_counts = vec![
        GemCount { floss: "310".to_string(), count: 100, hex: "000000".to_string() },
        GemCount { floss: "B5200".to_string(), count: 50, hex: "FFFFFF".to_string() },
    ];

    let start_time = Instant::now();
    let text_image_url = generate_text_image(&gem_counts).unwrap();
    let duration = start_time.elapsed();

    println!("generate_text_image took: {:?}", duration);

    assert!(!text_image_url.is_empty(), "Generated text image URL should not be empty");
}

#[test]
fn test_generate_gem_art_invalid_input() {
    let invalid_image_data = "data:image/png;base64,invalid_base64_string";
    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FFFFFF".to_string(), r: 255, g: 255, b: 255, value: "#FFFFFF".to_string() },
    ];
    let margin_mm = 10.0;
    let fit_option = ImageFitOption::Fit;
    let custom_width_mm = Some(210.0);
    let custom_height_mm = Some(297.0);

    let result = generate_gem_art(
        invalid_image_data,
        &colors,
        margin_mm,
        &fit_option,
        custom_width_mm,
        custom_height_mm,
    );

    assert!(result.is_err(), "generate_gem_art should return an error for invalid input");
    let error_message = result.unwrap_err();
    println!("Actual error message: {}", error_message);
    assert!(error_message.contains("Invalid image data") || error_message.contains("InvalidLength") || error_message.contains("Invalid first character") || error_message.contains("Encoded text cannot have a 6-bit remainder"), "Error message should indicate invalid image data");
}

#[test]
fn test_generate_gem_art_output_verification() {
    // Create a 1x1 red image
    let mut img = DynamicImage::new_rgba8(1, 1);
    img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));

    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    let image_data_url = format!("data:image/png;base64,{}", encoded_data);

    // Define a single red DMC color
    let colors = vec![
        Color { floss_number: "666".to_string(), hex: "FF0000".to_string(), r: 255, g: 0, b: 0, value: "#FF0000".to_string() },
    ];
    let margin_mm = 0.0;
    let fit_option = ImageFitOption::Fit;
    let custom_width_mm = Some(2.7);
    let custom_height_mm = Some(2.7);

    let (gem_image_url, gem_counts) = generate_gem_art(
        &image_data_url,
        &colors,
        margin_mm,
        &fit_option,
        custom_width_mm,
        custom_height_mm,
    ).unwrap();

    // Assert gem_counts
    assert_eq!(gem_counts.len(), 1, "Expected one gem count entry");
    assert_eq!(gem_counts[0].floss, "666", "Expected floss number 666");
    assert_eq!(gem_counts[0].count, 1, "Expected gem count of 1");
    assert_eq!(gem_counts[0].hex, "FF0000", "Expected hex color FF0000");

    // Decode the generated gem image to check its dimensions
    let decoded_gem_image_data = general_purpose::STANDARD.decode(gem_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_gem_image_data).unwrap();

    // Calculate expected dimensions based on gem_size_mm and dpi (300 dpi is used in lib.rs)
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;
    let expected_gem_size_px = ((2.7f32) * pixels_per_mm).round() as u32;

    assert_eq!(generated_img.width(), expected_gem_size_px, "Generated image width mismatch");
    assert_eq!(generated_img.height(), expected_gem_size_px, "Generated image height mismatch");
}

#[test]
fn test_generate_gem_art_fit_option() {
    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FF0000".to_string(), r: 255, g: 0, b: 0, value: "#FF0000".to_string() },
        Color { floss_number: "2".to_string(), hex: "00FF00".to_string(), r: 0, g: 255, b: 0, value: "#00FF00".to_string() },
    ];
    let margin_mm = 0.0;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    // Scenario 1: Landscape input image (100x50px) and Portrait canvas (210x297mm)
    let mut landscape_img = DynamicImage::new_rgba8(100, 50);
    landscape_img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Just a dummy pixel
    let mut buf = Vec::new();
    landscape_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let landscape_img_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let custom_width_mm_portrait = Some(210.0);
    let custom_height_mm_portrait = Some(297.0);

    let (gem_image_url, _) = generate_gem_art(
        &landscape_img_data_url,
        &colors,
        margin_mm,
        &ImageFitOption::Fit,
        custom_width_mm_portrait,
        custom_height_mm_portrait,
    ).unwrap();

    let decoded_gem_image_data = general_purpose::STANDARD.decode(gem_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_gem_image_data).unwrap();

    // Replicate internal logic of generate_gem_art for canvas dimensions
    let mut canvas_width_mm_s1 = custom_width_mm_portrait.unwrap();
    let mut canvas_height_mm_s1 = custom_height_mm_portrait.unwrap();
    let img_width_s1 = 100; // From landscape_img
    let img_height_s1 = 50; // From landscape_img

    let is_image_landscape_s1 = img_width_s1 > img_height_s1;
    let is_canvas_landscape_s1 = canvas_width_mm_s1 > canvas_height_mm_s1;

    if is_image_landscape_s1 != is_canvas_landscape_s1 {
        std::mem::swap(&mut canvas_width_mm_s1, &mut canvas_height_mm_s1);
    }

    let a4_width_px_s1 = (canvas_width_mm_s1 * pixels_per_mm as f32).round() as u32;
    let a4_height_px_s1 = (canvas_height_mm_s1 * pixels_per_mm as f32).round() as u32;

    assert_eq!(generated_img.width(), a4_width_px_s1, "Landscape image final canvas width mismatch");
    assert_eq!(generated_img.height(), a4_height_px_s1, "Landscape image final canvas height mismatch");

    // Scenario 2: Portrait input image (50x100px) and Landscape canvas (297x210mm)
    let mut portrait_img = DynamicImage::new_rgba8(50, 100);
    portrait_img.put_pixel(0, 0, Rgba([0, 255, 0, 255])); // Just a dummy pixel
    let mut buf = Vec::new();
    portrait_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let portrait_img_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let custom_width_mm_landscape = Some(297.0);
    let custom_height_mm_landscape = Some(210.0);

    let (gem_image_url, _) = generate_gem_art(
        &portrait_img_data_url,
        &colors,
        margin_mm,
        &ImageFitOption::Fit,
        custom_width_mm_landscape,
        custom_height_mm_landscape,
    ).unwrap();

    let decoded_gem_image_data = general_purpose::STANDARD.decode(gem_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_gem_image_data).unwrap();

    // Replicate internal logic of generate_gem_art for canvas dimensions
    let mut canvas_width_mm_s2 = custom_width_mm_landscape.unwrap();
    let mut canvas_height_mm_s2 = custom_height_mm_landscape.unwrap();
    let img_width_s2 = 50; // From portrait_img
    let img_height_s2 = 100; // From portrait_img

    let is_image_landscape_s2 = img_width_s2 > img_height_s2;
    let is_canvas_landscape_s2 = canvas_width_mm_s2 > canvas_height_mm_s2;

    if is_image_landscape_s2 != is_canvas_landscape_s2 {
        std::mem::swap(&mut canvas_width_mm_s2, &mut canvas_height_mm_s2);
    }

    let a4_width_px_s2 = (canvas_width_mm_s2 * pixels_per_mm as f32).round() as u32;
    let a4_height_px_s2 = (canvas_height_mm_s2 * pixels_per_mm as f32).round() as u32;

    assert_eq!(generated_img.width(), a4_width_px_s2, "Portrait image final canvas width mismatch");
    assert_eq!(generated_img.height(), a4_height_px_s2, "Portrait image final canvas height mismatch");
}

#[test]
fn test_generate_gem_art_crop_option() {
    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FF0000".to_string(), r: 255, g: 0, b: 0, value: "#FF0000".to_string() },
        Color { floss_number: "2".to_string(), hex: "00FF00".to_string(), r: 0, g: 255, b: 0, value: "#00FF00".to_string() },
    ];
    let margin_mm = 0.0;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    // Scenario 1: Landscape input image (100x50px) and Portrait canvas (210x297mm)
    let mut landscape_img = DynamicImage::new_rgba8(100, 50);
    landscape_img.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Just a dummy pixel
    let mut buf = Vec::new();
    landscape_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let landscape_img_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let custom_width_mm_portrait = Some(210.0);
    let custom_height_mm_portrait = Some(297.0);

    let (gem_image_url, _) = generate_gem_art(
        &landscape_img_data_url,
        &colors,
        margin_mm,
        &ImageFitOption::Crop,
        custom_width_mm_portrait,
        custom_height_mm_portrait,
    ).unwrap();

    let decoded_gem_image_data = general_purpose::STANDARD.decode(gem_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_gem_image_data).unwrap();

    // Replicate internal logic of generate_gem_art for canvas dimensions
    let mut canvas_width_mm_s1 = custom_width_mm_portrait.unwrap();
    let mut canvas_height_mm_s1 = custom_height_mm_portrait.unwrap();
    let img_width_s1 = 100; // From landscape_img
    let img_height_s1 = 50; // From landscape_img

    let is_image_landscape_s1 = img_width_s1 > img_height_s1;
    let is_canvas_landscape_s1 = canvas_width_mm_s1 > canvas_height_mm_s1;

    if is_image_landscape_s1 != is_canvas_landscape_s1 {
        std::mem::swap(&mut canvas_width_mm_s1, &mut canvas_height_mm_s1);
    }

    let a4_width_px_s1 = (canvas_width_mm_s1 * pixels_per_mm as f32).round() as u32;
    let a4_height_px_s1 = (canvas_height_mm_s1 * pixels_per_mm as f32).round() as u32;

    assert_eq!(generated_img.width(), a4_width_px_s1, "Landscape image crop width mismatch");
    assert_eq!(generated_img.height(), a4_height_px_s1, "Landscape image crop height mismatch");

    // Scenario 2: Portrait input image (50x100px) and Landscape canvas (297x210mm)
    let mut portrait_img = DynamicImage::new_rgba8(50, 100);
    portrait_img.put_pixel(0, 0, Rgba([0, 255, 0, 255])); // Just a dummy pixel
    let mut buf = Vec::new();
    portrait_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let portrait_img_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let custom_width_mm_landscape = Some(297.0);
    let custom_height_mm_landscape = Some(210.0);

    let (gem_image_url, _) = generate_gem_art(
        &portrait_img_data_url,
        &colors,
        margin_mm,
        &ImageFitOption::Crop,
        custom_width_mm_landscape,
        custom_height_mm_landscape,
    ).unwrap();

    let decoded_gem_image_data = general_purpose::STANDARD.decode(gem_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_gem_image_data).unwrap();

    // Replicate internal logic of generate_gem_art for canvas dimensions
    let mut canvas_width_mm_s2 = custom_width_mm_landscape.unwrap();
    let mut canvas_height_mm_s2 = custom_height_mm_landscape.unwrap();
    let img_width_s2 = 50; // From portrait_img
    let img_height_s2 = 100; // From portrait_img

    let is_image_landscape_s2 = img_width_s2 > img_height_s2;
    let is_canvas_landscape_s2 = canvas_width_mm_s2 > canvas_height_mm_s2;

    if is_image_landscape_s2 != is_canvas_landscape_s2 {
        std::mem::swap(&mut canvas_width_mm_s2, &mut canvas_height_mm_s2);
    }

    let a4_width_px_s2 = (canvas_width_mm_s2 * pixels_per_mm as f32).round() as u32;
    let a4_height_px_s2 = (canvas_height_mm_s2 * pixels_per_mm as f32).round() as u32;

    assert_eq!(generated_img.width(), a4_width_px_s2, "Portrait image crop width mismatch");
    assert_eq!(generated_img.height(), a4_height_px_s2, "Portrait image crop height mismatch");
}

#[test]
fn test_generate_gem_art_margin_application() {
    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FF0000".to_string(), r: 255, g: 0, b: 0, value: "#FF0000".to_string() },
    ];
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;
    let gem_size_mm = 2.7;
    let gem_size_px = ((gem_size_mm as f32) * pixels_per_mm).round() as u32;

    let custom_width_mm = Some(100.0);
    let custom_height_mm = Some(100.0);

    // Create a dummy 100x100px image
    let mut img = DynamicImage::new_rgba8(100, 100);
    img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let image_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    // Test with 0mm margin
    let margin_mm_0 = 0.0;
    let (_, gem_counts_0) = generate_gem_art(
        &image_data_url,
        &colors,
        margin_mm_0,
        &ImageFitOption::Fit,
        custom_width_mm,
        custom_height_mm,
    ).unwrap();

    let canvas_width_px_0 = (custom_width_mm.unwrap() * pixels_per_mm as f32).round() as u32;
    let canvas_height_px_0 = (custom_height_mm.unwrap() * pixels_per_mm as f32).round() as u32;
    let printable_width_px_0 = canvas_width_px_0 - (2 * (margin_mm_0 * pixels_per_mm as f32).round() as u32);
    let printable_height_px_0 = canvas_height_px_0 - (2 * (margin_mm_0 * pixels_per_mm as f32).round() as u32);
    let expected_num_gems_x_0 = printable_width_px_0 / gem_size_px;
    let expected_num_gems_y_0 = printable_height_px_0 / gem_size_px;
    assert_eq!(gem_counts_0[0].count, expected_num_gems_x_0 * expected_num_gems_y_0, "0mm margin: Total gem count mismatch");

    // Test with 10mm margin
    let margin_mm_10 = 10.0;
    let (_, gem_counts_10) = generate_gem_art(
        &image_data_url,
        &colors,
        margin_mm_10,
        &ImageFitOption::Fit,
        custom_width_mm,
        custom_height_mm,
    ).unwrap();

    let canvas_width_px_10 = (custom_width_mm.unwrap() * pixels_per_mm as f32).round() as u32;
    let canvas_height_px_10 = (custom_height_mm.unwrap() * pixels_per_mm as f32).round() as u32;
    let margin_px_10 = (margin_mm_10 * pixels_per_mm as f32).round() as u32;
    let printable_width_px_10 = canvas_width_px_10 - (2 * margin_px_10);
    let printable_height_px_10 = canvas_height_px_10 - (2 * margin_px_10);
    let expected_num_gems_x_10 = printable_width_px_10 / gem_size_px;
    let expected_num_gems_y_10 = printable_height_px_10 / gem_size_px;
    assert_eq!(gem_counts_10[0].count, expected_num_gems_x_10 * expected_num_gems_y_10, "10mm margin: Total gem count mismatch");

    // Assert that gem count with margin is less than without margin
    assert!(gem_counts_10[0].count < gem_counts_0[0].count, "Gem count with margin should be less than without margin");
}

#[test]
fn test_generate_gem_art_edge_cases() {
    let colors = vec![
        Color { floss_number: "1".to_string(), hex: "FF0000".to_string(), r: 255, g: 0, b: 0, value: "#FF0000".to_string() },
    ];
    let margin_mm = 0.0;
    let fit_option = ImageFitOption::Fit;

    // Scenario 1: Image too small (1x1px) for any gem to be generated
    let mut small_img = DynamicImage::new_rgba8(1, 1);
    small_img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    let mut buf = Vec::new();
    small_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let small_image_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let result = generate_gem_art(
        &small_image_data_url,
        &colors,
        margin_mm,
        &fit_option,
        Some(0.1), // Very small custom width
        Some(0.1), // Very small custom height
    );
    assert!(result.is_err(), "Should return error for image too small");
    assert!(result.unwrap_err().contains("Image dimensions are too small"), "Error message should indicate image dimensions are too small");

    // Scenario 2: Margins too large, leaving no printable area
    let mut normal_img = DynamicImage::new_rgba8(100, 100);
    normal_img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    let mut buf = Vec::new();
    normal_img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png).expect("Failed to write image to buffer");
    let normal_image_data_url = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&buf));

    let result = generate_gem_art(
        &normal_image_data_url,
        &colors,
        100.0, // Very large margin
        &fit_option,
        Some(10.0), // Small custom width
        Some(10.0), // Small custom height
    );
    assert!(result.is_err(), "Should return error for margins too large");
    assert!(result.unwrap_err().contains("Image dimensions are too small"), "Error message should indicate image dimensions are too small");
}

#[test]
fn test_generate_text_image_column_layout() {
    let mut gem_counts = Vec::new();
    for i in 1..=50 { // 50 entries should span multiple columns
        gem_counts.push(GemCount {
            floss: format!("DMC شیخ{}", i),
            count: i as u32,
            hex: "000000".to_string(),
        });
    }

    let start_time = Instant::now();
    let text_image_url = generate_text_image(&gem_counts).unwrap();
    let duration = start_time.elapsed();

    println!("generate_text_image_column_layout took: {:?}", duration);

    assert!(!text_image_url.is_empty(), "Generated text image URL should not be empty");

    // Decode the generated text image to check its dimensions
    let decoded_text_image_data = general_purpose::STANDARD.decode(text_image_url.split(",").nth(1).unwrap()).unwrap();
    let generated_img = image::load_from_memory(&decoded_text_image_data).unwrap();

    // Replicate canvas dimensions from generate_text_image
    let a4_width_mm = 210.0;
    let a4_height_mm = 297.0;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    let expected_width_px = (a4_width_mm as f32 * pixels_per_mm as f32).round() as u32;
    let expected_height_px = (a4_height_mm as f32 * pixels_per_mm as f32).round() as u32;

    assert!((generated_img.width() as f32 - expected_width_px as f32).abs() < 2.0, "Generated text image width mismatch");
    assert!((generated_img.height() as f32 - expected_height_px as f32).abs() < 2.0, "Generated text image height mismatch");
}

#[test]
fn test_get_dmc_colors_correctness() {
    use yew_project::dmc_colors;

    let colors = dmc_colors::get_dmc_colors();

    // Assert that colors are loaded and not empty
    assert!(!colors.is_empty(), "DMC colors should not be empty");

    // Assert a few known colors
    let color_310 = colors.iter().find(|c| c.floss == "310").expect("DMC 310 not found");
    assert_eq!(color_310.hex, "000000", "DMC 310 hex mismatch");
    assert_eq!(color_310.r, 0, "DMC 310 red mismatch");
    assert_eq!(color_310.g, 0, "DMC 310 green mismatch");
    assert_eq!(color_310.b, 0, "DMC 310 blue mismatch");

    let color_b5200 = colors.iter().find(|c| c.floss == "B5200 ").expect("DMC B5200 not found");
    assert_eq!(color_b5200.hex.to_uppercase(), "FFFFFF", "DMC B5200 hex mismatch");
    assert_eq!(color_b5200.r, 255, "DMC B5200 red mismatch");
    assert_eq!(color_b5200.g, 255, "DMC B5200 green mismatch");
    assert_eq!(color_b5200.b, 255, "DMC B5200 blue mismatch");
}