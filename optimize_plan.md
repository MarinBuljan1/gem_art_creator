# Optimization Plan for `image_processing.rs`

This document outlines the plan to optimize the `generate_gem_art` function in `src/image_processing.rs`.

## Performance Analysis

The `generate_gem_art` function is computationally intensive due to several factors:

1.  **Heavy Image Manipulation:** The function performs multiple resize, crop, and pixel manipulation operations, which are memory and CPU intensive.
2.  **Pixel-by-Pixel Color Matching:** The code iterates through each pixel of the source image to find the nearest DMC color using a k-d tree.
3.  **Repetitive Drawing Operations:** For each "gem" in the output image, the code draws a filled rectangle, a circle, and text. These drawing operations are slow, especially for images with a large number of gems.

## Optimization Strategy

### Defer Drawing Operations for Preview

*   **Goal:** Improve the perceived performance of the application by showing a preview of the gem art as quickly as possible.
*   **Action:**
    1.  **Separate the drawing of the gems from the drawing of the circles and letters.** The `generate_gem_art` function will be split into two functions:
        *   `generate_gem_art_preview`: This function will generate the gem art image *without* the circles and letters. This will be used to display the preview on the screen.
        *   `generate_gem_art_final`: This function will take the `gem_grid` and the `letter_map` as input and will draw the circles and letters on the gem art image. This will be the image that gets downloaded.
    2.  **Modify the frontend to call the new functions.** The frontend will first call `generate_gem_art_preview` to get the preview image and display it. Then, when the user clicks the download button, it will call `generate_gem_art_final` to get the final image and download it.
    3. **Verify**
    Verify that the new download button downloads the image with the circle and text and that the gem_art_legend continues to download.