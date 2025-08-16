# Upload Button Click Flow and Code Explanation

This document explains what happens when you click the "Upload Image" button, which components and code are involved, and how they work together.

## 1. The "Upload Image" Button

- **File:** `src/components/file_input_buttons.rs`
- **Component:** `FileInputButtons`
- **Code:**
  ```rust
  #[function_component(FileInputButtons)]
  pub fn file_input_buttons(props: &FileInputButtonsProps) -> Html {
      html! {
          <div class={classes!("section", "flex-row-around")} style="margin-bottom: 20px;">
              <input ref={props.file_input_ref.clone()} type="file" onchange={props.on_file_change.clone()} style="display: none;" />
              <button onclick={props.on_upload_button_click.clone()}>{ "Upload Image" }</button>
              // ... other buttons
          </div>
      }
  }
  ```
- **What it does:** This component renders the "Upload Image" button. The `onclick` event is wired to the `on_upload_button_click` callback, which is passed in as a prop from the parent `App` component. There's also a hidden file input element, which is used to actually open the file dialog.

## 2. The Main Application Component

- **File:** `src/components/mod.rs`
- **Component:** `App`
- **Code:**
  ```rust
  #[function_component(App)]
  pub fn app() -> Html {
      // ... other state variables

      let file_input_ref = use_node_ref();

      let on_upload_button_click = {
          let file_input_ref = file_input_ref.clone();
          Callback::from(move |_| {
              if let Some(input) = file_input_ref.cast::<web_sys::HtmlInputElement>() {
                  input.click();
              }
          })
      };

      let on_file_change = {
          // ... (see below)
      };

      html! {
          // ...
          <FileInputButtons
              file_input_ref={file_input_ref.clone()}
              on_file_change={on_file_change.clone()}
              on_upload_button_click={on_upload_button_click.clone()}
              // ... other props
          />
          // ...
      }
  }
  ```
- **What it does:** The `App` component is the main component of the application.
  - It defines the `on_upload_button_click` callback. This callback gets a reference to the hidden file input element and programmatically clicks it. This is a common technique for creating a custom-styled file upload button.
  - It also defines the `on_file_change` callback, which is triggered when the user selects a file from the file dialog.

## 3. Handling the File Input

- **File:** `src/components/mod.rs`
- **Component:** `App`
- **Code:**
  ```rust
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
  ```
- **What it does:** The `on_file_change` callback is where the selected image file is actually handled.
  - It gets the file from the input event.
  - It uses `gloo_file::callbacks::read_as_data_url` to read the file and encode it as a base64 data URL.
  - It then updates the `image_data` state with this data URL.

## 4. Image Processing

- **File:** `src/components/mod.rs`
- **Hook:** `use_effect_with_deps`
- **Code:**
  ```rust
  use_effect_with_deps(
      move |(image_data, selected_dmc_colors, margin_mm, image_fit_option, custom_width_mm, custom_height_mm)| {
          // ... (filters colors)

          if let Some(image_data) = (*image_data).as_ref() {
              match generate_gem_art_preview(image_data, &colors_for_generation, **margin_mm, image_fit_option, **custom_width_mm, **custom_height_mm) {
                  Ok((preview_data, counts, gem_art_data)) => {
                      generated_image_data_for_effect.set(Some(preview_data));
                      gem_counts_for_effect.set(counts);
                      gem_art_data_state_for_effect.set(Some(gem_art_data));
                  }
                  Err(_e) => {
                      // Handle error
                  }
              }
          }
      },
      (image_data.clone(), selected_dmc_colors.clone(), margin_mm.clone(), image_fit_option.clone(), custom_width_mm.clone(), custom_height_mm.clone()),
  );
  ```
- **What it does:** This `use_effect_with_deps` hook is a special function that runs whenever one of its dependencies (in this case, `image_data`) changes.
  - When the `image_data` state is updated with the new image, this hook is triggered.
  - It then calls the `generate_gem_art_preview` function from `src/image_processing.rs`, passing it the image data and the currently selected DMC colors.

## 5. The Core Logic: `generate_gem_art_preview`

- **File:** `src/image_processing.rs`
- **Function:** `generate_gem_art_preview`
- **What it does:** This is where the main image processing happens.
  1.  **Decodes the image:** It takes the base64 data URL, decodes it, and loads it into an image object.
  2.  **Resizes the image:** It resizes the image to the final dimensions of the gem art. This is a crucial optimization, as it means the color matching is done on a much smaller image.
  3.  **Finds the nearest color:** It iterates through each pixel of the resized image. For each pixel, it finds the closest color from the list of **user-selected** DMC colors. This is the most computationally intensive part of the process. To make this search faster, the code uses a k-d tree, which is a data structure that is very efficient for finding nearest neighbors.
  4.  **Generates the preview:** It creates a new image, the "gem art preview," where each "gem" is a colored square corresponding to the closest DMC color.
  5.  **Counts the gems:** It counts how many gems of each color are needed.
  6.  **Returns the results:** It returns the preview image, the gem counts, and other data needed for the final download.

## Integrating `color_map.json`

Your `color_map.json` file is intended to speed up the color matching process. The idea is to have a pre-calculated map of which color in the image maps to which DMC color.

However, the current implementation has a dynamic aspect that makes this tricky: the user can select a *subset* of the available DMC colors. The color matching is done only against this user-selected subset.

A pre-generated `color_map.json` would likely map each possible color to its closest color in the *entire* DMC palette. To use this, you would need to:

1.  Load `color_map.json` at the start.
2.  In the pixel processing loop, you would first look up the pixel color in your map to find the nearest color in the full DMC palette.
3.  Then, you would need to check if this nearest color is in the user's *selected* list of colors.
4.  If it is, you use it. If not, you would have to find the nearest color from the user's selection. This adds complexity.

**Conclusion:**

While using a pre-calculated color map is a good idea in principle, the current implementation is already highly optimized using a k-d tree and by processing a resized image. Given the dynamic nature of the color selection, integrating `color_map.json` might not result in a significant performance improvement and would add considerable complexity to the code. The main bottleneck is the nearest-neighbor search, which is already handled efficiently.
