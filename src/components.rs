use wasm_bindgen::prelude::*;
use yew::prelude::*;
use gloo_file::File;
use gloo_file::callbacks::FileReader;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::JsCast;
use std::collections::HashSet;
use crate::dmc_colors;
use crate::image_processing::{generate_gem_art, generate_text_image};
use crate::models::{Color, GemCount, ImageFitOption};
use crate::utils::to_excel_column;

#[function_component(App)]
pub fn app() -> Html {
    let dmc_colors = use_state(|| dmc_colors::get_dmc_colors());
    let selected_dmc_colors = use_state(|| {
        dmc_colors::get_dmc_colors().into_iter().map(|c| c.floss).collect::<HashSet<String>>()
    });
    let sort_by_number = use_state(|| false);
    let is_settings_open = use_state(|| false);
    let margin_mm = use_state(|| 30.0);
    let custom_width_mm = use_state(|| Some(210.0));
    let custom_height_mm = use_state(|| Some(297.0));
    let is_help_modal_open = use_state(|| false);
    let image_fit_option = use_state(|| ImageFitOption::Fit);

    let on_sort_by_color_click = {
        let sort_by_number = sort_by_number.clone();
        Callback::from(move |_| {
            sort_by_number.set(false);
        })
    };

    let on_settings_click = {
        let is_settings_open = is_settings_open.clone();
        Callback::from(move |_| {
            is_settings_open.set(!*is_settings_open);
        })
    };

    let on_help_icon_mouseover = {
        let is_help_modal_open = is_help_modal_open.clone();
        Callback::from(move |_| {
            is_help_modal_open.set(true);
        })
    };

    let on_help_icon_mouseout = {
        let is_help_modal_open = is_help_modal_open.clone();
        Callback::from(move |_| {
            is_help_modal_open.set(false);
        })
    };

    let on_help_icon_click = {
        let is_help_modal_open = is_help_modal_open.clone();
        Callback::from(move |_| {
            is_help_modal_open.set(!*is_help_modal_open);
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
        move |(image_data, selected_dmc_colors, margin_mm, image_fit_option, custom_width_mm, custom_height_mm)| {
            let current_dmc_colors = dmc_colors_for_effect.clone();
            let colors_for_generation: Vec<Color> = selected_dmc_colors
                .iter()
                .filter_map(|floss| {
                    current_dmc_colors.iter().find(|dmc_color| &dmc_color.floss == floss)
                })
                .map(|dmc_color| Color {
                    value: "#".to_string() + &dmc_color.hex,
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
                match generate_gem_art(image_data, &colors_for_generation, **margin_mm, image_fit_option, **custom_width_mm, **custom_height_mm) {
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
        (image_data.clone(), selected_dmc_colors.clone(), margin_mm.clone(), image_fit_option.clone(), custom_width_mm.clone(), custom_height_mm.clone()),
    );

    let download = {
        let generated_image_data = generated_image_data.clone();
        let gem_counts = gem_counts.clone();
        Callback::from(move |_| {
            if let Some(data) = (*generated_image_data).as_ref() {
                let document = web_sys::window().unwrap().document().unwrap();
                let link = document.create_element("a").unwrap();
                let link: web_sys::HtmlAnchorElement = link.dyn_into().unwrap();
                link.set_href(data);
                link.set_download("gem_art.png");
                link.click();
            }

            if let Ok(text_image_data) = generate_text_image(&gem_counts) {
                let document = web_sys::window().unwrap().document().unwrap();
                let link = document.create_element("a").unwrap();
                let link: web_sys::HtmlAnchorElement = link.dyn_into().unwrap();
                link.set_href(&text_image_data);
                link.set_download("gem_art_legend.png");
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
        <div class={classes!("main-container")}>
            <div class={classes!("left-panel")}>
                <h1>{ "Gem Art Creator" }</h1>
                <div class={classes!("section", "flex-row-around")} style="margin-bottom: 20px;">
                    <input ref={file_input_ref} type="file" onchange={on_file_change} style="display: none;" />
                    <button onclick={on_upload_button_click}>{ "Upload Image" }</button>
                    <button onclick={download} disabled={(*generated_image_data).is_none()}>{ "Download" }</button>
                    <button onclick={on_settings_click} class={classes!("settings-button")}>{ "⚙️" }</button>
                </div>
                { if *is_settings_open {
                    html! {
                        <div class={classes!("section", "settings")}>
                            <div class={classes!("setting")}>
                                <label for="margin_mm">{ "Margin (mm)" }</label>
                                <input type="number" id="margin_mm" value={margin_mm.to_string()} onchange={Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    margin_mm.set(input.value().parse().unwrap_or(30.0));
                                })} min="0" />
                            </div>
                            <div class={classes!("setting")}>
                                <div class={classes!("page-sizing-input-group")}>
                                    <label>{ "Page Sizing:" }</label>
                                    <input type="number" id="custom_width_mm" value={custom_width_mm.as_ref().map_or("".to_string(), |w| w.to_string())} onchange={Callback::from(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        custom_width_mm.set(input.value().parse().ok());
                                    })} min="0" />
                                    <label>{ "x" }</label>
                                    <input type="number" id="custom_height_mm" value={custom_height_mm.as_ref().map_or("".to_string(), |h| h.to_string())} onchange={Callback::from(move |e: Event| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        custom_height_mm.set(input.value().parse().ok());
                                    })} min="0" />
                                    <span class={classes!("help-icon")} onmouseover={on_help_icon_mouseover} onmouseout={on_help_icon_mouseout} onclick={on_help_icon_click}>{ "?" }</span>
                                    { if *is_help_modal_open {
                                        html! {
                                            <div class={classes!("help-modal")}>
                                                <h3>{ "Standard Paper Sizes" }</h3>
                                                <ul>
                                                    <li>{ "A5: 148 x 210 mm" }</li>
                                                    <li>{ "A4: 210 x 297 mm" }</li>
                                                    <li>{ "A3: 297 x 420 mm" }</li>
                                                    <li>{ "A2: 420 x 594 mm" }</li>
                                                    <li>{ "A1: 594 x 841 mm" }</li>
                                                    <li>{ "A0: 841 x 1189 mm" }</li>
                                                    <li>{ "4x6\" : 101.6 x 152.4 mm" }</li>
                                                    <li>{ "5x5\" : 127 x 127 mm" }</li>
                                                    <li>{ "5x7\" : 127 x 177.8 mm" }</li>
                                                    <li>{ "6x6\" : 152.4 x 152.4 mm" }</li>
                                                    <li>{ "6x8\" : 152.4 x 203.2 mm" }</li>
                                                    <li>{ "8x8\" : 203.2 x 203.2 mm" }</li>
                                                    <li>{ "8x10\" : 203.2 x 254 mm" }</li>
                                                    <li>{ "12x12\" : 304.8 x 304.8 mm" }</li>
                                                    <li>{ "16x20\" : 406.4 x 508 mm" }</li>
                                                </ul>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    } }
                                </div>
                            </div>
                            <div class={classes!("setting")}>
                                <label>{ "Image Fit" }</label>
                                <div class={classes!("radio-group")}>
                                    <div>
                                        <input type="radio" id="fit_entire" name="image_fit" value="fit" checked={*image_fit_option == ImageFitOption::Fit} onchange={{ 
                                            let image_fit_option = image_fit_option.clone();
                                            Callback::from(move |_| {
                                                image_fit_option.set(ImageFitOption::Fit)
                                            })
                                        }} />
                                        <label for="fit_entire">{ "Fit entire image into frame" }</label>
                                    </div>
                                    <div>
                                        <input type="radio" id="crop_to_fit" name="image_fit" value="crop" checked={*image_fit_option == ImageFitOption::Crop} onchange={{ 
                                            let image_fit_option = image_fit_option.clone();
                                            Callback::from(move |_| {
                                                image_fit_option.set(ImageFitOption::Crop)
                                            })
                                        }} />
                                        <label for="crop_to_fit">{ "Crop image to fit frame" }</label>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                                        html! {}
                } }
                <div class={classes!("section", "colours")}>
                    <div class={classes!("flex-row-around")}>
                        <div class={classes!("sort-buttons")}>
                            <button onclick={on_sort_by_color_click} disabled={!*sort_by_number}>{ "Sort by Colour" }</button>
                            <button onclick={on_sort_by_number_click} disabled={*sort_by_number}>{ "Sort by Number" }</button>
                        </div>
                        <div class={classes!("select-buttons")}>
                            <button onclick={on_select_all_click}>{ "Select All" }</button>
                            <button onclick={on_deselect_all_click}>{ "Deselect All" }</button>
                        </div>
                    </div>
                    <div class={classes!("color-grid")}>
                        { for {
                            let mut sorted_dmc_colors = (*dmc_colors).clone();
                            if *sort_by_number {
                                sorted_dmc_colors.sort_by(|a, b| {
                                    let a_num = a.floss.parse::<u32>();
                                    let b_num = b.floss.parse::<u32>();

                                    match (a_num, b_num) {
                                        (Ok(a_val), Ok(b_val)) => a_val.cmp(&b_val),
                                        (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                                        (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                                        (Err(_), Err(_)) => a.floss.cmp(&b.floss),
                                    }
                                });
                            }
                            sorted_dmc_colors.into_iter().map(|dmc_color| {
                                let floss = dmc_color.floss.clone();
                                let is_selected = selected_dmc_colors.contains(&floss);
                                let background_style = format!("background-color: #{}", dmc_color.hex);
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
                    <div class={classes!("text-output-container")}>
                        { for (*gem_counts).iter().enumerate().map(|(i, count)| {
                            let letter = to_excel_column(i + 1);
                            let circle_style = format!("background-color: #{}", count.hex);
                            html! {
                                <div class={classes!("gem-count-line")}>
                                    <span class={classes!("gem-count-circle")} style={circle_style}>{ letter }</span>
                                    <span>{ format!(" #{}: {} gems", count.floss, count.count) }</span>
                                </div>
                            }
                        }) }
                    </div>
                </div>
            </div>
            <div class={classes!("right-panel")}>
                <canvas id="preview-canvas"></canvas>
            </div>
        </div>
    }
}
