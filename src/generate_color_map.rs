use std::collections::HashMap;
use std::fs;
use std::io::Write;
use palette::{Srgb, Lab, IntoColor};
use kiddo::KdTree;
use yew_project::models::DmcColorPrecomputed;

// This function is a modified version of init_dmc_colors_data from image_processing.rs
fn load_dmc_colors_data() -> Result<(Vec<DmcColorPrecomputed>, KdTree<f32, usize, 3>), String> {
    let file_content = include_str!("dmc_colors_precomputed.json");
    let precomputed_colors: Vec<DmcColorPrecomputed> = serde_json::from_str(file_content)
        .map_err(|e| format!("Failed to parse dmc_colors_precomputed.json: {}", e))?;

    let mut kdtree = KdTree::new();
    for (i, color) in precomputed_colors.iter().enumerate() {
        let _ = kdtree.add(&[color.lab_l, color.lab_a, color.lab_b], i);
    }

    Ok((precomputed_colors, kdtree))
}

fn main() -> Result<(), String> {
    let (dmc_colors, kdtree) = load_dmc_colors_data()?;

    let mut color_map: HashMap<String, usize> = HashMap::new();

    println!("Generating color map...");

    for r_orig in 0..=255 {
        for g_orig in 0..=255 {
            for b_orig in 0..=255 {
                let r = ((r_orig as f32 / 3.0).round() * 3.0) as u8;
                let g = ((g_orig as f32 / 3.0).round() * 3.0) as u8;
                let b = ((b_orig as f32 / 3.0).round() * 3.0) as u8;
                let key = format!("{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8);

                if !color_map.contains_key(&key) {
                    let srgb_pixel = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                    let lab_pixel: Lab = srgb_pixel.into_color();

                    let nearest_neighbor = kdtree
                        .nearest_one(&[lab_pixel.l, lab_pixel.a, lab_pixel.b], &kiddo::distance::squared_euclidean)
                        .unwrap();
                    let closest_color_index = *nearest_neighbor.1;
                    let closest_dmc_color = &dmc_colors[closest_color_index];

                    color_map.insert(key, closest_color_index);
                }
            }
        }
    }

    println!("Color map generated. Serializing to JSON...");

    let output_file = "color_map.json";
    let json_content = serde_json::to_string_pretty(&color_map)
        .map_err(|e| format!("Failed to serialize color map to JSON: {}", e))?;

    let mut file = fs::File::create(output_file)
        .map_err(|e| format!("Failed to create file {}: {}", output_file, e))?;
    file.write_all(json_content.as_bytes())
        .map_err(|e| format!("Failed to write to file {}: {}", output_file, e))?;

    let file_size = fs::metadata(output_file)
        .map_err(|e| format!("Failed to get file metadata for {}: {}", output_file, e))?.len();

    println!("Color map saved to {}. File size: {} bytes", output_file, file_size);

    Ok(())
}