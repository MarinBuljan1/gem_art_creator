use criterion::{criterion_group, criterion_main, Criterion};
use yew_project::image_processing::{generate_gem_art, generate_text_image};
use yew_project::models::{ImageFitOption, Color, GemCount};
use yew_project::dmc_colors;
use image::{ImageBuffer, Rgba};
use std::time::Duration;
use base64::{engine::general_purpose, Engine as _};

fn generate_test_image(width: u32, height: u32) -> String {
    let mut img = ImageBuffer::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba([((x % 256) as u8), ((y % 256) as u8), 0, 255]);
    }
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).unwrap();
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    format!("data:image/png;base64,{}", encoded_data)
}

fn benchmark_generate_gem_art_image_size(c: &mut Criterion) {
    let dmc_colors = dmc_colors::get_dmc_colors();
    let colors: Vec<Color> = dmc_colors.iter().map(|c| Color {
        value: format!("#{}", c.hex),
        floss_number: c.floss.clone(),
        r: c.r,
        g: c.g,
        b: c.b,
        hex: c.hex.clone(),
    }).collect();

    let mut group = c.benchmark_group("generate_gem_art_image_size");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    for size in [100, 300, 500].iter() {
        let image_data = generate_test_image(*size, *size);
        group.bench_function(format!("{}x{}", size, size), |b| b.iter(|| {
            generate_gem_art(
                &image_data,
                &colors,
                10.0, // margin_mm
                &ImageFitOption::Fit, // fit_option
                None, // custom_width_mm
                None, // custom_height_mm
            ).unwrap();
        }));
    }
    group.finish();
}

fn benchmark_generate_gem_art_color_count(c: &mut Criterion) {
    let dmc_colors = dmc_colors::get_dmc_colors();
    let colors: Vec<Color> = dmc_colors.iter().map(|c| Color {
        value: format!("#{}", c.hex),
        floss_number: c.floss.clone(),
        r: c.r,
        g: c.g,
        b: c.b,
        hex: c.hex.clone(),
    }).collect();
    
    let image_data = generate_test_image(300, 300);

    let mut group = c.benchmark_group("generate_gem_art_color_count");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    for count in [10, 100, 500].iter() {
        let color_subset: Vec<Color> = colors.iter().take(*count).cloned().collect();
        group.bench_function(format!("{} colors", count), |b| b.iter(|| {
            generate_gem_art(
                &image_data,
                &color_subset,
                10.0, // margin_mm
                &ImageFitOption::Fit, // fit_option
                None, // custom_width_mm
                None, // custom_height_mm
            ).unwrap();
        }));
    }
    group.finish();
}

fn benchmark_generate_gem_art_fit_vs_crop(c: &mut Criterion) {
    let dmc_colors = dmc_colors::get_dmc_colors();
    let colors: Vec<Color> = dmc_colors.iter().map(|c| Color {
        value: format!("#{}", c.hex),
        floss_number: c.floss.clone(),
        r: c.r,
        g: c.g,
        b: c.b,
        hex: c.hex.clone(),
    }).collect();
    
    let image_data = generate_test_image(500, 300); // Landscape image

    let mut group = c.benchmark_group("fit_vs_crop");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("Fit", |b| b.iter(|| {
        generate_gem_art(
            &image_data,
            &colors,
            10.0, // margin_mm
            &ImageFitOption::Fit,
            None, // custom_width_mm
            None, // custom_height_mm
        ).unwrap();
    }));

    group.bench_function("Crop", |b| b.iter(|| {
        generate_gem_art(
            &image_data,
            &colors,
            10.0, // margin_mm
            &ImageFitOption::Crop,
            None, // custom_width_mm
            None, // custom_height_mm
        ).unwrap();
    }));

    group.finish();
}

fn benchmark_generate_text_image(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_text_image");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    for count in [10, 50, 100, 200, 400, 500].iter() {
        let gem_counts: Vec<GemCount> = (0..*count).map(|i| GemCount {
            floss: i.to_string(),
            count: i as u32,
            hex: "000000".to_string(),
        }).collect();

        group.bench_function(format!("{} gem_counts", count), |b| b.iter(|| {
            generate_text_image(&gem_counts).unwrap();
        }));
    }
    group.finish();
}

criterion_group!(benches, benchmark_generate_gem_art_image_size, benchmark_generate_gem_art_color_count, benchmark_generate_gem_art_fit_vs_crop, benchmark_generate_text_image);
criterion_main!(benches);
