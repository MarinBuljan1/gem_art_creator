use wasm_bindgen::prelude::*;
use yew::prelude::*;
use gloo_file::File;
use gloo_file::callbacks::FileReader;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;
use image::{GenericImageView, DynamicImage, Rgba, imageops::FilterType, GenericImage};
use base64::{engine::general_purpose, Engine as _};
use deltae::{DeltaE, LabValue, DE2000};
use palette::{Srgb, Lab, IntoColor, FromColor};
use std::collections::{HashSet, HashMap};

mod dmc_colors;

#[derive(Clone, PartialEq)]
struct GemCount {
    floss: String,
    count: u32,
    hex: String,
}

fn to_excel_column(num: usize) -> String {
    let mut s = String::new();
    let mut n = num;
    while n > 0 {
        let rem = (n - 1) % 26;
        s.insert(0, (b'A' + rem as u8) as char);
        n = (n - 1) / 26;
    }
    s
}

fn generate_gem_art(image_data: &str, colors: &Vec<Color>) -> Result<(String, Vec<GemCount>), String> {
    let base64_data = image_data.split(",").nth(1).ok_or("Invalid image data")?;
    let decoded_data = general_purpose::STANDARD.decode(base64_data).map_err(|e| e.to_string())?;
    let img = image::load_from_memory(&decoded_data).map_err(|e| e.to_string())?;

    let mut a4_width_mm = 210.0;
    let mut a4_height_mm = 297.0;
    let margin_mm = 30.0;
    let gem_size_mm = 2.7;
    let dpi = 300.0;
    let mm_per_inch = 25.4;
    let pixels_per_mm = dpi / mm_per_inch;

    let (img_width, img_height) = img.dimensions();

    // Adjust A4 dimensions if the image is landscape
    if img_width > img_height {
        std::mem::swap(&mut a4_width_mm, &mut a4_height_mm);
    }

    let a4_width_px = ((a4_width_mm * pixels_per_mm) as f32).round() as u32;
    let a4_height_px = ((a4_height_mm * pixels_per_mm) as f32).round() as u32;
    let margin_px = ((margin_mm * pixels_per_mm) as f32).round() as u32;

    let printable_width_mm = a4_width_mm - (2.0 * margin_mm);
    let printable_height_mm = a4_height_mm - (2.0 * margin_mm);

    let aspect_ratio = img_width as f32 / img_height as f32;

    let (new_width_mm, new_height_mm) = if printable_width_mm / aspect_ratio <= printable_height_mm {
        (printable_width_mm, printable_width_mm / aspect_ratio)
    } else {
        (printable_height_mm * aspect_ratio, printable_height_mm)
    };

    let num_gems_x = (new_width_mm / gem_size_mm).floor() as u32;
    let num_gems_y = (new_height_mm / gem_size_mm).floor() as u32;

    let resized_img = img.resize_exact(num_gems_x, num_gems_y, FilterType::Nearest);

    let gem_colors: Vec<Lab> = colors
        .iter()
        .map(|c| {
            let srgb: Srgb<f32> = Srgb::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0);
            srgb.into_linear().into_color()
        })
        .collect();

    let mut color_counts: HashMap<String, (u32, String)> = HashMap::new();

    let gem_pixels_on_final_image = (gem_size_mm * pixels_per_mm).round() as u32;
    let gem_art_width_px = num_gems_x * gem_pixels_on_final_image;
    let gem_art_height_px = num_gems_y * gem_pixels_on_final_image;

    let mut gem_art_image = DynamicImage::new_rgba8(gem_art_width_px, gem_art_height_px);

    for gx in 0..num_gems_x {
        for gy in 0..num_gems_y {
            let pixel = resized_img.get_pixel(gx, gy);
            let srgb_pixel = Srgb::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);
            let lab_pixel: Lab = srgb_pixel.into_color();

            let mut closest_color_index = 0;
            let mut min_distance = f32::MAX;

            for (i, color) in gem_colors.iter().enumerate() {
                let distance = DeltaE::new(
                    LabValue::new(lab_pixel.l, lab_pixel.a, lab_pixel.b).unwrap(),
                    LabValue::new(color.l, color.a, color.b).unwrap(),
                    DE2000,
                )
                .value;
                if distance < min_distance {
                    min_distance = distance;
                    closest_color_index = i;
                }
            }
            
            let color_info = &colors[closest_color_index];
            let entry = color_counts.entry(color_info.floss_number.clone()).or_insert((0, color_info.hex.clone()));
            entry.0 += 1;

            let closest_color = &gem_colors[closest_color_index];
            let srgb_color: Srgb<u8> = Srgb::from_color(*closest_color).into_format();
            let (r, g, b) = srgb_color.into_components();
            let gem_rgba = Rgba([r, g, b, 255]);

            for px in 0..gem_pixels_on_final_image {
                for py in 0..gem_pixels_on_final_image {
                    gem_art_image.put_pixel(
                        gx * gem_pixels_on_final_image + px,
                        gy * gem_pixels_on_final_image + py,
                        gem_rgba,
                    );
                }
            }
        }
    }

    let mut final_image = DynamicImage::new_rgba8(a4_width_px, a4_height_px);
    // Fill with white background
    for x in 0..a4_width_px {
        for y in 0..a4_height_px {
            final_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    // Calculate top-left corner to paste the gem art
    let available_width_px = a4_width_px - (2 * margin_px);
    let available_height_px = a4_height_px - (2 * margin_px);

    let offset_x = (available_width_px - gem_art_width_px) / 2;
    let offset_y = (available_height_px - gem_art_height_px) / 2;

    let paste_x = margin_px + offset_x;
    let paste_y = margin_px + offset_y;

    // Paste the gem art onto the final image
    image::imageops::overlay(&mut final_image, &gem_art_image, paste_x as i64, paste_y as i64);

    let mut buf = Vec::new();
    final_image.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageOutputFormat::Png).map_err(|e| e.to_string())?;
    let encoded_data = general_purpose::STANDARD.encode(&buf);
    let image_data_url = format!("data:image/png;base64,{}", encoded_data);

    let mut sorted_counts: Vec<_> = color_counts.into_iter().map(|(floss, (count, hex))| GemCount { floss, count, hex }).collect();
    sorted_counts.sort_by(|a, b| b.count.cmp(&a.count));

    Ok((image_data_url, sorted_counts))
}

#[derive(Clone, PartialEq, Default)]
struct Color {
    // id: usize, // Removed
    value: String, // This will store the hex color string
    floss_number: String,
    // Add RGB components for direct use in generate_gem_art
    r: u8,
    g: u8,
    b: u8,
    hex: String,
}

#[function_component(App)]
fn app() -> Html {
    let dmc_colors = use_state(|| dmc_colors::get_dmc_colors()); // Corrected import
    let selected_dmc_colors = use_state(|| {
        dmc_colors::get_dmc_colors().into_iter().map(|c| c.floss).collect::<HashSet<String>>()
    });
    let sort_by_number = use_state(|| false);

    let on_sort_by_color_click = {
        let sort_by_number = sort_by_number.clone();
        Callback::from(move |_| {
            sort_by_number.set(false);
        })
    };

    let on_sort_by_number_click = {
        let sort_by_number = sort_by_number.clone();
        Callback::from(move |_| {
            sort_by_number.set(true);
        })
    };

    let on_select_all_click = {
        let selected_dmc_colors = selected_dmc_colors.clone();
        let dmc_colors = dmc_colors.clone();
        Callback::from(move |_| {
            let all_floss_numbers = (*dmc_colors).iter().map(|c| c.floss.clone()).collect();
            selected_dmc_colors.set(all_floss_numbers);
        })
    };

    let on_deselect_all_click = {
        let selected_dmc_colors = selected_dmc_colors.clone();
        Callback::from(move |_| {
            selected_dmc_colors.set(HashSet::<String>::new());
        })
    };

    let on_dmc_color_click = {
        let selected_dmc_colors = selected_dmc_colors.clone();
        Callback::from(move |floss: String| {
            let mut current_selection = (*selected_dmc_colors).clone();
            if current_selection.contains(&floss) {
                current_selection.remove(&floss);
            } else {
                current_selection.insert(floss);
            }
            selected_dmc_colors.set(current_selection);
        })
    };
    let image_file = use_state::<Option<File>, _>(|| None);
    let image_data = use_state::<Option<String>, _>(|| None);
    let generated_image_data = use_state::<Option<String>, _>(|| None);
    let gem_counts = use_state::<Vec<GemCount>, _>(|| vec![]);
    let reader = use_state::<Option<FileReader>, _>(|| None);

    let file_input_ref = use_node_ref();

    let on_upload_button_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                let input = input.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(0).await;
                    input.click();
                });
            }
        })
    };


    let on_file_change = {
        let image_file = image_file.clone();
        let image_data = image_data.clone();
        let reader = reader.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file = File::from(web_sys::File::from(file));
                    let image_data = image_data.clone();
                    let task = gloo_file::callbacks::read_as_data_url(&file, move |res| {
                        image_data.set(Some(res.unwrap()));
                    });
                    reader.set(Some(task));
                    image_file.set(Some(file));
                }
            }
        })
    };

    let generated_image_data_for_effect = generated_image_data.clone();
    let gem_counts_for_effect = gem_counts.clone();
    let dmc_colors_for_effect = dmc_colors.clone();
    use_effect_with_deps(
        move |(image_data, selected_dmc_colors)| {
            let current_dmc_colors = dmc_colors_for_effect.clone();
            let colors_for_generation: Vec<Color> = selected_dmc_colors
                .iter()
                .filter_map(|floss| {
                    current_dmc_colors.iter().find(|dmc_color| &dmc_color.floss == floss)
                })
                .map(|dmc_color| Color {
                    value: format!("#{}", dmc_color.hex),
                    floss_number: dmc_color.floss.clone(),
                    r: dmc_color.r,
                    g: dmc_color.g,
                    b: dmc_color.b,
                    hex: dmc_color.hex.clone(),
                })
                .collect();

            if colors_for_generation.is_empty() {
                generated_image_data_for_effect.set(None);
                gem_counts_for_effect.set(vec![]);
                return;
            }

            if let Some(image_data) = (*image_data).as_ref() {
                match generate_gem_art(image_data, &colors_for_generation) {
                    Ok((data, counts)) => {
                        generated_image_data_for_effect.set(Some(data));
                        gem_counts_for_effect.set(counts);
                    }
                    Err(_e) => {
                        // Handle error
                    }
                }
            }
        },
        (image_data.clone(), selected_dmc_colors.clone()),
    );

    let download = {
        let generated_image_data = generated_image_data.clone();
        Callback::from(move |_| {
            if let Some(data) = (*generated_image_data).as_ref() {
                let document = web_sys::window().unwrap().document().unwrap();
                let link = document.create_element("a").unwrap();
                let link: web_sys::HtmlAnchorElement = link.dyn_into().unwrap();
                link.set_href(data);
                link.set_download("gem_art.png");
                link.click();
            }
        })
    };

    use_effect_with_deps(
        move |generated_image_data| {
            if let Some(data) = generated_image_data {
                let document = web_sys::window().unwrap().document().unwrap();
                let canvas = document.get_element_by_id("preview-canvas").unwrap();
                let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                let image = web_sys::HtmlImageElement::new().unwrap();
                image.set_src(data);
                let context = context.clone();
                let image_clone = image.clone();
                let onload = Closure::wrap(Box::new(move || {
                    canvas.set_width(image_clone.width());
                    canvas.set_height(image_clone.height());
                    context.draw_image_with_html_image_element(&image_clone, 0.0, 0.0).unwrap();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
            }
        },
        (*generated_image_data).clone(),
    );

    html! {
        <div class="main-container">
            <div class="left-panel">
                <h1>{ "Gem Art Creator" }</h1>
                <div class="section flex-row-around" style="margin-bottom: 20px;">
                    <input ref={file_input_ref} type="file" onchange={on_file_change} style="display: none;" />
                    <button onclick={on_upload_button_click}>{ "Upload Image" }</button>
                    <button onclick={download} disabled={(*generated_image_data).is_none()}>{ "Download" }</button>
                </div>
                <div class="section colours">
                    <div class="flex-row-around">
                        <div class="sort-buttons">
                            <button onclick={on_sort_by_color_click} disabled={!*sort_by_number}>{ "Sort by Colour" }</button>
                            <button onclick={on_sort_by_number_click} disabled={*sort_by_number}>{ "Sort by Number" }</button>
                        </div>
                        <div class="select-buttons">
                            <button onclick={on_select_all_click}>{ "Select All" }</button>
                            <button onclick={on_deselect_all_click}>{ "Deselect All" }</button>
                        </div>
                    </div>
                    <div class="color-grid">
                        { for {
                            let mut sorted_dmc_colors = (*dmc_colors).clone();
                            if *sort_by_number {
                                sorted_dmc_colors.sort_by(|a, b| a.floss.cmp(&b.floss));
                            }
                            sorted_dmc_colors.into_iter().map(|dmc_color| {
                                let floss = dmc_color.floss.clone();
                                let is_selected = selected_dmc_colors.contains(&floss);
                                let background_style = format!("background-color: #{};", dmc_color.hex);
                                html! {
                                    <div
                                        key={floss.clone()}
                                        class={classes!("color-item", is_selected.then_some("selected"))}
                                        style={background_style}
                                        onclick={on_dmc_color_click.reform(move |_| floss.clone())}
                                    >
                                        { &dmc_color.floss }
                                    </div>
                                }
                            })
                        } }
                    </div>
                    <div class="text-output-container">
                        { for (*gem_counts).iter().enumerate().map(|(i, count)| {
                            let letter = to_excel_column(i + 1);
                            let circle_style = format!("background-color: #{};", count.hex);
                            html! {
                                <div class="gem-count-line">
                                    <span class="gem-count-circle" style={circle_style}>{ letter }</span>
                                    <span>{ format!(" #{}: {} gems", count.floss, count.count) }</span>
                                </div>
                            }
                        }) }
                    </div>
                </div>
            </div>
            <div class="right-panel">
                <canvas id="preview-canvas"></canvas>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}