# Optimization Plan for `image_processing.rs`

This document outlines the plan to optimize the `generate_gem_art` function in `src/image_processing.rs`.

## Performance Analysis

The `generate_gem_art` function is computationally intensive due to several factors:

1.  **Heavy Image Manipulation:** The function performs multiple resize, crop, and pixel manipulation operations, which are memory and CPU intensive.
2.  **Pixel-by-Pixel Color Matching:** The code iterates through each pixel of the source image to find the nearest DMC color using a k-d tree.
3.  **Repetitive Drawing Operations:** For each "gem" in the output image, the code draws a filled rectangle, a circle, and text. These drawing operations are slow, especially for images with a large number of gems.

## Optimization Strategy

The optimization strategy is divided into two phases. The first phase, which has already been completed, involved parallelizing the color matching loop using `rayon`. The second phase will focus on more advanced optimizations.

### Phase 1: Parallelize Color Matching (Completed)

*   **Action:** Parallelized the color matching loop using `rayon`'s parallel iterators (`par_iter`).
*   **Result:** Significant performance improvement, especially for larger images and more colors.

### Phase 2: Advanced Optimizations (Future Work)

The following optimizations can be implemented to further improve performance:

1.  **Reduce Intermediate Image Creation (Completed):**
    *   **Goal:** Reduce memory usage and improve performance by avoiding unnecessary data copying.
    *   **Action:** Removed the intermediate `gem_art_image` and drew the gems directly on the `final_image`.
    *   **Result:** Significant performance improvement (11-13%) across all `generate_gem_art` benchmarks.

2.  **Use a More Efficient Drawing Method (Skipped):**
    *   **Goal:** Speed up the drawing process.
    *   **Action:** For drawing simple shapes like filled rectangles, directly manipulate the image buffer instead of using the `imageproc` drawing functions. This involves writing the pixel data for the rectangles directly into the image's pixel buffer.
    *   **Justification:** The `imageproc` drawing functions are likely already well-optimized. Replacing them with manual implementations is a complex task that is unlikely to yield a significant performance improvement.

3.  **Parallelize the Drawing Loop (Skipped):**
    *   **Goal:** Further leverage parallel processing to speed up the image generation.
    *   **Action:** Re-attempt to parallelize the drawing loop.
    *   **Justification:** The drawing operations are not easily parallelizable because they all modify the same `final_image`. Using a `Mutex` to protect the image results in a performance regression due to high contention. Other approaches are very complex and have a high risk of introducing new bugs with a low probability of a significant performance gain.

This plan will be executed iteratively, with performance measurements taken at each stage to evaluate the impact of the changes.