use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HelpModalProps {
    pub is_help_modal_open: UseStateHandle<bool>,
    pub on_help_icon_mouseover: Callback<MouseEvent>,
    pub on_help_icon_mouseout: Callback<MouseEvent>,
    pub on_help_icon_click: Callback<MouseEvent>,
}

#[function_component(HelpModal)]
pub fn help_modal(props: &HelpModalProps) -> Html {
    html! {
        <>
            <span class={classes!("help-icon")} onmouseover={props.on_help_icon_mouseover.clone()} onmouseout={props.on_help_icon_mouseout.clone()} onclick={props.on_help_icon_click.clone()}>{ "?" }</span>
            { if *props.is_help_modal_open {
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
        </>
    }
}