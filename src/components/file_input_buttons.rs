use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputButtonsProps {
    pub file_input_ref: NodeRef,
    pub on_file_change: Callback<Event>,
    pub on_upload_button_click: Callback<MouseEvent>,
    pub download: Callback<MouseEvent>,
    pub generated_image_data_is_none: bool,
    pub on_settings_click: Callback<MouseEvent>,
}

#[function_component(FileInputButtons)]
pub fn file_input_buttons(props: &FileInputButtonsProps) -> Html {
    html! {
        <div class={classes!("section", "flex-row-around")} style="margin-bottom: 20px;">
            <input ref={props.file_input_ref.clone()} type="file" onchange={props.on_file_change.clone()} style="display: none;" />
            <button onclick={props.on_upload_button_click.clone()}>{ "Upload Image" }</button>
            <button onclick={props.download.clone()} disabled={props.generated_image_data_is_none}>{ "Download" }</button>
            <button onclick={props.on_settings_click.clone()} class={classes!("settings-button")}>{ "⚙️" }</button>
        </div>
    }
}