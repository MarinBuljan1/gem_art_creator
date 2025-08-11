# Optimization Plan for `image_processing.rs`

This document outlines the plan to optimize the `generate_gem_art` function in `src/image_processing.rs`.

## Performance Analysis

The `generate_gem_art` function is computationally intensive due to several factors:

1.  **Heavy Image Manipulation:** The function performs multiple resize, crop, and pixel manipulation operations, which are memory and CPU intensive.
2.  **Pixel-by-Pixel Color Matching:** The code iterates through each pixel of the source image to find the nearest DMC color using a k-d tree.
3.  **Repetitive Drawing Operations:** For each "gem" in the output image, the code draws a filled rectangle, a circle, and text. These drawing operations are slow, especially for images with a large number of gems.

## Optimization Strategy

The primary optimization strategy is to introduce parallelism to the pixel processing loops using the `rayon` crate. This will allow the computationally intensive tasks to be spread across multiple CPU cores, leading to a significant performance improvement.

### Step-by-Step Plan

1.  **Establish a Performance Baseline:**
    *   Run `cargo bench` to get a baseline performance measurement of the current implementation.

2.  **Introduce `rayon` for Parallel Processing:**
    *   Add `rayon` as a dependency in `Cargo.toml`.
    *   Modify the pixel processing loops in `generate_gem_art` to use `rayon`'s parallel iterators (`par_iter`). The main loops to target are:
        *   The loop that finds the nearest color for each pixel of the resized image.
        *   The loop that draws the gems on the final `gem_art_image`.

3.  **Measure Performance After Parallelization:**
    *   Run `cargo bench` again to measure the performance improvement after introducing `rayon`.

4.  **Further Optimization (Future Work):**
    *   **Reduce Intermediate Image Creation:** Investigate ways to reduce the number of intermediate image buffers created during the image processing pipeline.
    *   **Pre-render Gem Templates:** For gems of the same size, pre-render a single gem template and copy it to the final image instead of drawing each gem individually.
    *   **Optimize Drawing Operations:** Explore more performant ways to draw shapes, such as by directly manipulating the image buffer.

This plan will be executed iteratively, with performance measurements taken at each stage to evaluate the impact of the changes.
**Run `cargo check`** to ensure the code compiles without errors.
**Run `cargo test`** to ensure all existing tests pass.
**Run `cargo bench`** to measure the performance impact of the change.