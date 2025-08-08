use yew::prelude::*;
use std::collections::HashSet;
use crate::dmc_colors::DmcColor;

#[derive(Properties, PartialEq)]
pub struct ColorSelectionPanelProps {
    pub dmc_colors: UseStateHandle<Vec<DmcColor>>,
    pub selected_dmc_colors: UseStateHandle<HashSet<String>>,
    pub sort_by_number: UseStateHandle<bool>,
    pub on_sort_by_color_click: Callback<MouseEvent>,
    pub on_sort_by_number_click: Callback<MouseEvent>,
    pub on_select_all_click: Callback<MouseEvent>,
    pub on_deselect_all_click: Callback<MouseEvent>,
    pub on_dmc_color_click: Callback<String>,
}

#[function_component(ColorSelectionPanel)]
pub fn color_selection_panel(props: &ColorSelectionPanelProps) -> Html {
    html! {
        <div class={classes!("section", "colours")}>
            <div class={classes!("flex-row-around")}>
                <div class={classes!("sort-buttons")}>
                    <button onclick={props.on_sort_by_color_click.clone()} disabled={!*props.sort_by_number}>{ "Sort by Colour" }</button>
                    <button onclick={props.on_sort_by_number_click.clone()} disabled={*props.sort_by_number}>{ "Sort by Number" }</button>
                </div>
                <div class={classes!("select-buttons")}>
                    <button onclick={props.on_select_all_click.clone()}>{ "Select All" }</button>
                    <button onclick={props.on_deselect_all_click.clone()}>{ "Deselect All" }</button>
                </div>
            </div>
            <div class={classes!("color-grid")}>
                { for {
                    let mut sorted_dmc_colors = (*props.dmc_colors).clone();
                    if *props.sort_by_number {
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
                        let is_selected = props.selected_dmc_colors.contains(&floss);
                        let background_style = format!("background-color: #{}", dmc_color.hex);
                        html! {
                            <div
                                key={floss.clone()}
                                class={classes!("color-item", is_selected.then_some("selected"))}
                                style={background_style}
                                onclick={props.on_dmc_color_click.reform(move |_| floss.clone())}
                            >
                                { &dmc_color.floss }
                            </div>
                        }
                    })
                } }
            </div>
        </div>
    }
}