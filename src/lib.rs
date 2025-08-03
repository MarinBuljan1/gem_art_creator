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
use std::str::FromStr;


fn generate_gem_art(image_data: &str, colors: &Vec<Color>) -> Result<String, String> {
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

    let (img_width, img_height) = img.dimensions();
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
            let srgb: Srgb<f32> = Srgb::from_str(&c.value).unwrap().into_format();
            srgb.into_linear().into_color()
        })
        .collect();

    let gem_pixels_on_final_image = (gem_size_mm * pixels_per_mm).round() as u32;
    let gem_art_width_px = num_gems_x * gem_pixels_on_final_image;
    let gem_art_height_px = num_gems_y * gem_pixels_on_final_image;

    let mut gem_art_image = DynamicImage::new_rgba8(gem_art_width_px, gem_art_height_px);

    for gx in 0..num_gems_x {
        for gy in 0..num_gems_y {
            let pixel = resized_img.get_pixel(gx, gy);
            let srgb_pixel = Srgb::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);
            let lab_pixel: Lab = srgb_pixel.into_color();

            let mut closest_color = &gem_colors[0];
            let mut min_distance = DeltaE::new(LabValue::new(lab_pixel.l, lab_pixel.a, lab_pixel.b).unwrap(), LabValue::new(closest_color.l, closest_color.a, closest_color.b).unwrap(), DE2000).value;

            for color in &gem_colors[1..] {
                let distance = DeltaE::new(LabValue::new(lab_pixel.l, lab_pixel.a, lab_pixel.b).unwrap(), LabValue::new(color.l, color.a, color.b).unwrap(), DE2000).value;
                if distance < min_distance {
                    min_distance = distance;
                    closest_color = color;
                }
            }

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
    Ok(format!("data:image/png;base64,{}", encoded_data))
}

#[derive(Clone, PartialEq, Default)]
struct Color {
    id: usize,
    value: String,
}

#[function_component(App)]
fn app() -> Html {
    let colors = use_state(|| vec![Color { id: 0, value: "#000000".to_string() }]);
    let next_color_id = use_state(|| 1);
    let image_file = use_state::<Option<File>, _>(|| None);
    let image_data = use_state::<Option<String>, _>(|| None);
    let generated_image_data = use_state::<Option<String>, _>(|| None);
    let reader = use_state::<Option<FileReader>, _>(|| None);

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

    let add_color = {
        let colors = colors.clone();
        let next_color_id = next_color_id.clone();
        Callback::from(move |_| {
            let mut new_colors = (*colors).clone();
            new_colors.push(Color {
                id: *next_color_id,
                value: "#000000".to_string(),
            });
            colors.set(new_colors);
            next_color_id.set(*next_color_id + 1);
        })
    };

    let delete_color = {
        let colors = colors.clone();
        Callback::from(move |id: usize| {
            let new_colors = (*colors).clone().into_iter().filter(|c| c.id != id).collect();
            colors.set(new_colors);
        })
    };

    let on_color_change = {
        let colors = colors.clone();
        Callback::from(move |(id, value): (usize, String)| {
            let new_colors = (*colors)
                .clone()
                .into_iter()
                .map(|c| {
                    if c.id == id {
                        Color { id, value: value.clone() }
                    } else {
                        c
                    }
                })
                .collect();
            colors.set(new_colors);
        })
    };

    

    let generated_image_data_for_effect = generated_image_data.clone();
    use_effect_with_deps(
        move |(image_data, colors)| {
            if let Some(image_data) = (*image_data).as_ref() {
                match generate_gem_art(image_data, colors) {
                    Ok(data) => generated_image_data_for_effect.set(Some(data)),
                    Err(_e) => {
                        // Handle error, e.g., display an alert
                        // alert(&e);
                    }
                }
            }
        },
        (image_data.clone(), colors.clone()),
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
        <div>
            <h1>{ "Gem Art Creator" }</h1>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <div>
                    <h2>{ "1. Upload Image" }</h2>
                    <input type="file" onchange={on_file_change} />
                    {if let Some(image_data) = (*image_data).as_ref() {
                        html! { <img src={image_data.clone()} width="200" /> }
                    } else {
                        html! {}
                    }}
                </div>
                <div>
                    <h2>{ "5. Download" }</h2>
                    <button onclick={download} disabled={(*generated_image_data).is_none()}>{ "Download" }</button>
                </div>
            </div>
            <div>
                <h2>{ "2. Available Gem Colors" }</h2>
                { for colors.iter().map(|c| {
                    let c = c.clone();
                    html! {
                    <div key={c.id}>
                        <input type="color" value={c.value.clone()} onchange={on_color_change.reform(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            (c.id, input.value())
                        })} />
                        <button onclick={delete_color.reform(move |_| c.id)}>{ "Delete" }</button>
                    </div>
                }}) }
                <button onclick={add_color}>{ "Add Color" }</button>
            </div>
            // <div>
            //     <h2>{ "3. Generate" }</h2>
            //     <button onclick={generate}>{ "Generate" }</button>
            // </div>
            <div>
                <h2>{ "4. Preview" }</h2>
                <canvas id="preview-canvas"></canvas>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
