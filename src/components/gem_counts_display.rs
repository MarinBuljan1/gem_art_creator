use yew::prelude::*;
use crate::models::GemCount;
use crate::utils::to_excel_column;

#[derive(Properties, PartialEq)]
pub struct GemCountsDisplayProps {
    pub gem_counts: UseStateHandle<Vec<GemCount>>,
}

#[function_component(GemCountsDisplay)]
pub fn gem_counts_display(props: &GemCountsDisplayProps) -> Html {
    html! {
        <div class={classes!("text-output-container")}>
            { for (*props.gem_counts).iter().enumerate().map(|(i, count)| {
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
    }
}
