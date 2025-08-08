use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::collections::HashSet;
use crate::dmc_colors;
use crate::image_processing::{generate_gem_art, generate_text_image};
use crate::models::{Color, GemCount, ImageFitOption};

mod help_modal;
mod file_input_buttons;
mod settings_panel;
mod color_selection_panel;
mod gem_counts_display;
use help_modal::HelpModal;
use file_input_buttons::FileInputButtons;
use settings_panel::SettingsPanel;
use color_selection_panel::ColorSelectionPanel;
use gem_counts_display::GemCountsDisplay;

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
    let image_file = use_state::<Option<gloo_file::File>, _>(|| None);
    let image_data = use_state::<Option<String>, _>(|| None);
    let generated_image_data = use_state::<Option<String>, _>(|| None);
    let gem_counts = use_state::<Vec<GemCount>, _>(|| vec![]);
    let reader = use_state::<Option<gloo_file::callbacks::FileReader>, _>(|| None);

    let file_input_ref = use_node_ref();

    let on_upload_button_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<web_sys::HtmlInputElement>() {
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
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file = gloo_file::File::from(web_sys::File::from(file));
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
                <FileInputButtons
                    file_input_ref={file_input_ref.clone()}
                    on_file_change={on_file_change.clone()}
                    on_upload_button_click={on_upload_button_click.clone()}
                    download={download.clone()}
                    generated_image_data_is_none={(*generated_image_data).is_none()}
                />
                <button onclick={on_settings_click} class={classes!("settings-button")}>{ "⚙️" }</button>
                { if *is_settings_open {
                    html! {
                        <SettingsPanel
                            is_settings_open={is_settings_open.clone()}
                            margin_mm={margin_mm.clone()}
                            custom_width_mm={custom_width_mm.clone()}
                            custom_height_mm={custom_height_mm.clone()}
                            is_help_modal_open={is_help_modal_open.clone()}
                            image_fit_option={image_fit_option.clone()}
                            on_help_icon_mouseover={on_help_icon_mouseover.clone()}
                            on_help_icon_mouseout={on_help_icon_mouseout.clone()}
                            on_help_icon_click={on_help_icon_click.clone()}
                        />
                    }
                } else {
                                        html! {}
                } }
                <ColorSelectionPanel
                    dmc_colors={dmc_colors.clone()}
                    selected_dmc_colors={selected_dmc_colors.clone()}
                    sort_by_number={sort_by_number.clone()}
                    on_sort_by_color_click={on_sort_by_color_click.clone()}
                    on_sort_by_number_click={on_sort_by_number_click.clone()}
                    on_select_all_click={on_select_all_click.clone()}
                    on_deselect_all_click={on_deselect_all_click.clone()}
                    on_dmc_color_click={on_dmc_color_click.clone()}
                />
                <GemCountsDisplay
                    gem_counts={gem_counts.clone()}
                />
            </div>
            <div class={classes!("right-panel")}>
                <canvas id="preview-canvas"></canvas>
            </div>
        </div>
    }
}