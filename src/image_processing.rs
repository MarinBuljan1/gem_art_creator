use image::{GenericImageView, DynamicImage, Rgba, imageops::FilterType, GenericImage};
use base64::{engine::general_purpose, Engine as _};
use palette::{Srgb, Lab, IntoColor};
use imageproc::drawing::{draw_hollow_circle_mut, draw_text_mut, draw_filled_circle_mut};
use rusttype::{Font, Scale};
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::OnceLock;
use kiddo::KdTree;
use crate::models::{ImageFitOption, GemCount, Color, DmcColorPrecomputed};
use crate::utils::to_excel_column;

static DMC_COLORS_DATA: OnceLock<(Vec<DmcColorPrecomputed>, KdTree<f32, usize, 3>)> = OnceLock::new();

fn init_dmc_colors_data() -> Result<(Vec<DmcColorPrecomputed>, KdTree<f32, usize, 3>), String> {
    let file_content = include_str!("../src/dmc_colors_precomputed.json");
    let precomputed_colors: Vec<DmcColorPrecomputed> = serde_json::from_str(file_content)
        .map_err(|e| format!("Failed to parse dmc_colors_precomputed.json: {}", e))?;

    let mut kdtree = KdTree::new();
    for (i, color) in precomputed_colors.iter().enumerate() {
        let _ = kdtree.add(&[color.lab_l, color.lab_a, color.lab_b], i);
    }

    Ok((precomputed_colors, kdtree))
}

pub fn generate_gem_art(image_data: &str, selected_colors: &Vec<Color>, margin_mm: f32, fit_option: &ImageFitOption, custom_width_mm: Option<f32>, custom_height_mm: Option<f32>) -> Result<(String, Vec<GemCount>), String> {
    let (all_dmc_colors, _kdtree) = DMC_COLORS_DATA.get_or_init(|| init_dmc_colors_data().expect("Failed to initialize DMC colors data"));

    // Filter precomputed colors based on selected_colors
    let mut filtered_dmc_colors: Vec<DmcColorPrecomputed> = Vec::new();
    let mut filtered_kdtree = KdTree::new();
    let mut floss_to_index_map: HashMap<String, usize> = HashMap::new();

    for (_i, selected_color) in selected_colors.iter().enumerate() {
        if let Some(dmc_color) = all_dmc_colors.iter().find(|c| c.floss.trim() == selected_color.floss_number.trim()) {
            let _ = filtered_kdtree.add(&[dmc_color.lab_l, dmc_color.lab_a, dmc_color.lab_b], filtered_dmc_colors.len());
            floss_to_index_map.insert(dmc_color.floss.clone(), filtered_dmc_colors.len());
            filtered_dmc_colors.push(dmc_color.clone());
        }
    }

    if filtered_dmc_colors.is_empty() {
        return Err("No DMC colors selected or found.".to_string());
    }

    let base64_data = image_data.split(",").nth(1).ok_or("Invalid image data")?;
    let decoded_data = general_purpose::STANDARD.decode(base64_data).map_err(|e| e.to_string())?;
    let img = image::load_from_memory(&decoded_data).map_err(|e| e.to_string())?;

    let mut canvas_width_mm = custom_width_mm.unwrap_or(210.0);
    let mut canvas_height_mm = custom_height_mm.unwrap_or(297.0);
    let gem_size_mm = 2.7;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    let (img_width, img_height) = img.dimensions();

    // Determine image orientation
    let is_image_landscape = img_width > img_height;

    // Determine initial canvas orientation (before potential swap)
    let is_canvas_landscape = canvas_width_mm > canvas_height_mm;

    // If orientations don\'t match, swap canvas dimensions to match image orientation
    if is_image_landscape != is_canvas_landscape {
        std::mem::swap(&mut canvas_width_mm, &mut canvas_height_mm);
    }

    let a4_width_px = ((canvas_width_mm * pixels_per_mm) as f32).round() as u32;
    let a4_height_px = ((canvas_height_mm * pixels_per_mm) as f32).round() as u32;
    let margin_px = ((margin_mm * pixels_per_mm) as f32).round() as u32;

    if 2 * margin_px >= a4_width_px || 2 * margin_px >= a4_height_px {
        return Err("Image dimensions are too small to generate gem art due to large margins.".to_string());
    }

    let printable_width_px = a4_width_px - (2 * margin_px);
    let printable_height_px = a4_height_px - (2 * margin_px);

    let img_aspect_ratio = img_width as f32 / img_height as f32;
    let printable_aspect_ratio = printable_width_px as f32 / printable_height_px as f32;

    let (final_img_width_px, final_img_height_px);
    let mut processed_img = img;

    match fit_option {
        ImageFitOption::Fit => {
            if img_aspect_ratio > printable_aspect_ratio {
                // Image is wider, fit by width
                final_img_width_px = printable_width_px;
                final_img_height_px = (printable_width_px as f32 / img_aspect_ratio).round() as u32;
            } else {
                // Image is taller or same aspect, fit by height
                final_img_height_px = printable_height_px;
                final_img_width_px = (printable_height_px as f32 * img_aspect_ratio).round() as u32;
            }
            processed_img = processed_img.resize_exact(final_img_width_px, final_img_height_px, FilterType::Nearest);
        },
        ImageFitOption::Crop => {
            if img_aspect_ratio > printable_aspect_ratio {
                // Image is wider, scale height to fill and crop width
                let scaled_height = printable_height_px;
                let scaled_width = (printable_height_px as f32 * img_aspect_ratio).round() as u32;
                processed_img = processed_img.resize_exact(scaled_width, scaled_height, FilterType::Nearest);

                let crop_x = (scaled_width - printable_width_px) / 2;
                processed_img = processed_img.crop_imm(crop_x, 0, printable_width_px, printable_height_px);
            } else {
                // Image is taller, scale width to fill and crop height
                let scaled_width = printable_width_px;
                let scaled_height = (printable_width_px as f32 / img_aspect_ratio).round() as u32;
                processed_img = processed_img.resize_exact(scaled_width, scaled_height, FilterType::Nearest);

                let crop_y = (scaled_height - printable_height_px) / 2;
                processed_img = processed_img.crop_imm(0, crop_y, printable_width_px, printable_height_px);
            }
            final_img_width_px = printable_width_px;
            final_img_height_px = printable_height_px;
        }
    }

    let gem_size_px = (gem_size_mm * pixels_per_mm).round() as u32;
    let num_gems_x = final_img_width_px / gem_size_px;
    let num_gems_y = final_img_height_px / gem_size_px;

    if num_gems_x == 0 || num_gems_y == 0 {
        return Err("Image dimensions are too small to generate gem art.".to_string());
    }

    let resized_img = processed_img.resize_exact(num_gems_x, num_gems_y, FilterType::Nearest);

    let gem_grid: Vec<usize> = (0..num_gems_x)
        .into_par_iter()
        .flat_map(|gx| (0..num_gems_y).into_par_iter().map(move |gy| (gx, gy)))
        .map(|(gx, gy)| {
            let pixel = resized_img.get_pixel(gx, gy);
            let srgb_pixel = Srgb::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
            );
            let lab_pixel: Lab = srgb_pixel.into_color();
    
            let nearest_neighbor = filtered_kdtree
                .nearest_one(&[lab_pixel.l, lab_pixel.a, lab_pixel.b], &kiddo::distance::squared_euclidean)
                .unwrap();
            *nearest_neighbor.1
        })
        .collect();

    let mut color_counts: HashMap<String, (u32, String)> = HashMap::new();
    for &closest_color_index in &gem_grid {
        let color_info = &filtered_dmc_colors[closest_color_index];
        let entry = color_counts.entry(color_info.floss.clone()).or_insert((0, color_info.hex.clone()));
        entry.0 += 1;
    }

    let mut sorted_counts: Vec<_> = color_counts.into_iter().map(|(floss, (count, hex))| GemCount { floss, count, hex }).collect();
    sorted_counts.sort_by(|a, b| b.count.cmp(&a.count));

    let letter_map: HashMap<String, String> = sorted_counts
        .iter()
        .enumerate()
        .map(|(i, gem_count)| (gem_count.floss.clone(), to_excel_column(i + 1)))
        .collect();

    let gem_pixels_on_final_image = (gem_size_mm * pixels_per_mm).round() as u32;
    let gem_art_width_px = num_gems_x * gem_pixels_on_final_image;
    let gem_art_height_px = num_gems_y * gem_pixels_on_final_image;
    let mut gem_art_image = DynamicImage::new_rgba8(gem_art_width_px, gem_art_height_px);
    let font_data = include_bytes!("../static/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data as &[_]).unwrap();

    for gx in 0..num_gems_x {
        for gy in 0..num_gems_y {
            let closest_color_index = gem_grid[(gx * num_gems_y + gy) as usize];
            let color_info = &filtered_dmc_colors[closest_color_index];
            let gem_rgba = Rgba([color_info.r, color_info.g, color_info.b, 255]);

            for px in 0..gem_pixels_on_final_image {
                for py in 0..gem_pixels_on_final_image {
                    gem_art_image.put_pixel(
                        gx * gem_pixels_on_final_image + px,
                        gy * gem_pixels_on_final_image + py,
                        gem_rgba,
                    );
                }
            }

            let center_x = (gx * gem_pixels_on_final_image + gem_pixels_on_final_image / 2) as i32;
            let center_y = (gy * gem_pixels_on_final_image + gem_pixels_on_final_image / 2) as i32;
            let radius = ((gem_pixels_on_final_image / 2) - 2) as i32;

            let blended_rgba = Rgba([color_info.blended_r, color_info.blended_g, color_info.blended_b, 255]);
            draw_hollow_circle_mut(&mut gem_art_image, (center_x, center_y), radius, blended_rgba);

            let letter = letter_map.get(&color_info.floss).unwrap();
            let scale = Scale::uniform(gem_pixels_on_final_image as f32 * 0.6);
            let v_metrics = font.v_metrics(scale);
            let glyphs: Vec<_> = font.layout(&letter, scale, rusttype::Point { x: 0.0, y: v_metrics.ascent }).collect();
            let glyphs_width = glyphs.iter().map(|g| g.pixel_bounding_box().unwrap().width() as f32).sum::<f32>();
            let text_x = center_x - (glyphs_width / 2.0) as i32;
            let text_y = center_y - (v_metrics.ascent - v_metrics.descent) as i32 / 2;
            draw_text_mut(&mut gem_art_image, blended_rgba, text_x, text_y, scale, &font, &letter);
        }
    }

    let mut final_image = DynamicImage::new_rgba8(a4_width_px, a4_height_px);
    for x in 0..a4_width_px {
        for y in 0..a4_height_px {
            final_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    let available_width_px = a4_width_px - (2 * margin_px);
    let available_height_px = a4_height_px - (2 * margin_px);
    let offset_x = (available_width_px - gem_art_width_px) / 2;
    let offset_y = (available_height_px - gem_art_height_px) / 2;
    let paste_x = margin_px + offset_x;
    let paste_y = margin_px + offset_y;

    image::imageops::overlay(&mut final_image, &gem_art_image, paste_x as i64, paste_y as i64);

    let mut buf = Vec::new();
    final_image.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).map_err(|e| e.to_string())?;
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    let image_data_url = format!("data:image/png;base64,{}", encoded_data);

    Ok((image_data_url, sorted_counts))
}

pub fn generate_text_image(gem_counts: &Vec<GemCount>) -> Result<String, String> {
    let a4_width_mm = 210.0;
    let a4_height_mm = 297.0;
    let margin_mm = 10.0;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    let a4_width_px = (a4_width_mm * pixels_per_mm) as u32;
    let a4_height_px = (a4_height_mm * pixels_per_mm) as u32;
    let margin_px = (margin_mm * pixels_per_mm) as u32;

    let mut text_image = DynamicImage::new_rgba8(a4_width_px, a4_height_px);
    for x in 0..a4_width_px {
        for y in 0..a4_height_px {
            text_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    let font_data = include_bytes!("../static/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data as &[_]).unwrap();
    let scale = Scale::uniform(48.0);
    let text_color = Rgba([0, 0, 0, 255]);
    let line_height = 80;
    let column_width = (a4_width_px - 2 * margin_px) / 3;

    let mut x = margin_px;
    let mut y = margin_px;
    let max_y = a4_height_px - margin_px;

    for (i, count) in gem_counts.iter().enumerate() {
        if y + line_height > max_y {
            y = margin_px;
            x += column_width;
        }

        let letter = to_excel_column(i + 1);
        let r = u8::from_str_radix(&count.hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&count.hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&count.hex[4..6], 16).unwrap();
        let circle_color = Rgba([r, g, b, 255]);

        let v_metrics = font.v_metrics(scale);
        let circle_y = y as i32 + (v_metrics.ascent - v_metrics.descent) as i32 / 2;

        draw_filled_circle_mut(&mut text_image, (x as i32 + 20, circle_y), 15, circle_color);
        draw_text_mut(&mut text_image, text_color, x as i32 + 50, y as i32, scale, &font, &letter);

        let line = format!(" - #{}: {} gems", count.floss, count.count);
        draw_text_mut(&mut text_image, text_color, x as i32 + 100, y as i32, scale, &font, &line);

        y += line_height;
    }

    let mut buf = Vec::new();
    text_image.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).map_err(|e| e.to_string())?;
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    let image_data_url = format!("data:image/png;base64,{}", encoded_data);

    Ok(image_data_url)
}
