use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::ImageFitOption;
use crate::components::HelpModal;

#[derive(Properties, PartialEq)]
pub struct SettingsPanelProps {
    pub is_settings_open: UseStateHandle<bool>,
    pub margin_mm: UseStateHandle<f32>,
    pub custom_width_mm: UseStateHandle<Option<f32>>,
    pub custom_height_mm: UseStateHandle<Option<f32>>,
    pub is_help_modal_open: UseStateHandle<bool>,
    pub image_fit_option: UseStateHandle<ImageFitOption>,
    pub on_help_icon_mouseover: Callback<MouseEvent>,
    pub on_help_icon_mouseout: Callback<MouseEvent>,
    pub on_help_icon_click: Callback<MouseEvent>,
    pub gem_size_mm: UseStateHandle<f32>,
    pub mapping_weight: UseStateHandle<f32>,
}

#[function_component(SettingsPanel)]
pub fn settings_panel(props: &SettingsPanelProps) -> Html {
    html! {
        { if *props.is_settings_open {
            html! {
                <div class={classes!("section", "settings")}>
                    <div class={classes!("setting")}>
                        <label for="margin_mm">{ "Margin (mm)" }</label>
                        <input type="number" id="margin_mm" value={props.margin_mm.to_string()} onchange={{
                            let margin_mm = props.margin_mm.clone();
                            Callback::from(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                margin_mm.set(input.value().parse().unwrap_or(30.0));
                            })
                        }} min="0" />
                    </div>
                    <div class={classes!("setting")}>
                        <label for="gem_size_mm">{ "Gem Size (mm)" }</label>
                        <input type="number" id="gem_size_mm" value={props.gem_size_mm.to_string()} onchange={{
                            let gem_size_mm = props.gem_size_mm.clone();
                            Callback::from(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                gem_size_mm.set(input.value().parse::<f32>().unwrap_or(2.7).max(0.1));
                            })
                        }} min="0.1" step="0.1" />
                    </div>
                    <div class={classes!("setting")}>
                        <div class={classes!("page-sizing-input-group")}>
                            <label style="font-weight: bold;">{ "Page Sizing:" }</label>
                            <input type="number" id="custom_width_mm" value={props.custom_width_mm.as_ref().map_or("".to_string(), |w| w.to_string())} onchange={{
                                let custom_width_mm = props.custom_width_mm.clone();
                                Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    custom_width_mm.set(input.value().parse().ok());
                                })
                            }} min="0" />
                            <label>{ "x" }</label>
                            <input type="number" id="custom_height_mm" value={props.custom_height_mm.as_ref().map_or("".to_string(), |h| h.to_string())} onchange={{
                                let custom_height_mm = props.custom_height_mm.clone();
                                Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    custom_height_mm.set(input.value().parse().ok());
                                })
                            }} min="0" />
                            <HelpModal
                                is_help_modal_open={props.is_help_modal_open.clone()}
                                on_help_icon_mouseover={props.on_help_icon_mouseover.clone()}
                                on_help_icon_mouseout={props.on_help_icon_mouseout.clone()}
                                on_help_icon_click={props.on_help_icon_click.clone()}
                            />
                        </div>
                    </div>
                    <div class={classes!("setting")}>
                        <label>{ "Image Fit" }</label>
                        <div class={classes!("radio-group")}>
                            <div>
                                <input type="radio" id="fit_entire" name="image_fit" value="fit" checked={*props.image_fit_option == ImageFitOption::Fit} onchange={{
                                    let image_fit_option = props.image_fit_option.clone();
                                    Callback::from(move |_| {
                                        image_fit_option.set(ImageFitOption::Fit)
                                    })
                                }} />
                                <label for="fit_entire">{ "Fit entire image into frame" }</label>
                            </div>
                            <div>
                                <input type="radio" id="crop_to_fit" name="image_fit" value="crop" checked={*props.image_fit_option == ImageFitOption::Crop} onchange={{
                                    let image_fit_option = props.image_fit_option.clone();
                                    Callback::from(move |_| {
                                        image_fit_option.set(ImageFitOption::Crop)
                                    })
                                }} />
                                <label for="crop_to_fit">{ "Crop image to fit frame" }</label>
                            </div>
                        </div>
                    </div>
                    <div class={classes!("setting")}>
                        <label for="mapping_weight">{ "Color mapping style" }</label>
                        <div class={classes!("slider-row")}>
                            <span>{ "Match closest color" }</span>
                            <input type="range" id="mapping_weight" min="0" max="1" step="0.05" value={props.mapping_weight.to_string()} onchange={{
                                let mapping_weight = props.mapping_weight.clone();
                                Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    let v = input.value().parse::<f32>().unwrap_or(0.0).clamp(0.0, 1.0);
                                    mapping_weight.set(v);
                                })
                            }} />
                            <span>{ "Balance tones across selected colors" }</span>
                        </div>
                    </div>
                    <div class={classes!("setting")}>
                        <a href="https://www.instructables.com/DIY-Diamond-Painting-Make-Your-Own-Simple-Adhesive/" target="_blank">{ "DIY Instructions" }</a>
                    </div>
                </div>
            }
        } else {
            html! {}
        } }
    }
}
