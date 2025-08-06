## Test Plan: Gem Art Creator Optimization

This document outlines the testing strategy to ensure the correctness and measure the performance of the `gem_art_creator` application, particularly focusing on the image processing and generation logic.

---

### I. Unit Tests for Correctness (`cargo test`)

Unit tests will focus on individual functions, especially `generate_gem_art` and `generate_text_image`, to verify their behavior under various conditions.

**1. `generate_gem_art` Function Tests:**

*   **Input Validation:**
    *   Test with invalid image data (e.g., malformed base64 string) to ensure proper error handling and graceful failure.
*   **Basic Output Verification:**
    *   Provide a small, known input image and a limited, predefined set of DMC colors.
    *   Assert that the output image dimensions are correct based on the input and canvas settings.
    *   Verify that the `GemCount` results (floss number, count, hex) match expected values for the given input.
    *   (Optional but Recommended): Implement checks for a few key pixel colors in the generated image to ensure color mapping is accurate.
*   **`ImageFitOption::Fit` Behavior:**
    *   Test with a landscape input image and a portrait canvas (default A4 or custom dimensions) to ensure the image is scaled down to fit entirely within the canvas while maintaining its aspect ratio.
    *   Test with a portrait input image and a landscape canvas to ensure similar behavior.
    *   Test with various `custom_width_mm` and `custom_height_mm` inputs, verifying the image correctly fits within the specified dimensions.
*   **`ImageFitOption::Crop` Behavior:**
    *   Test with a landscape input image and a portrait canvas to ensure the image is scaled to fill the canvas and then cropped to fit.
    *   Test with a portrait input image and a landscape canvas to ensure similar behavior.
    *   Test with various `custom_width_mm` and `custom_height_mm` inputs, verifying the image correctly fills and crops to the specified dimensions.
*   **Margin Application:**
    *   Verify that the `margin_mm` parameter correctly reduces the effective printable area of the canvas.
*   **Edge Cases:**
    *   Test scenarios where very small images or very large margins might result in `num_gems_x` or `num_gems_y` being zero, ensuring the function returns appropriate error messages.

**2. `generate_text_image` Function Tests:**

*   **Basic Output Verification:**
    *   Provide a small, known list of `GemCount` entries.
    *   Assert that the generated legend image contains the correct text (floss numbers, gem counts), colors (based on hex codes), and overall layout (e.g., circle positions, text positions).
*   **Column Layout:**
    *   Test with a larger number of `GemCount` entries to ensure the text correctly flows into multiple columns on the legend image as expected.

**3. `to_excel_column` Function Tests:**

*   Test various integer inputs to ensure correct Excel-style column name generation:
    *   `1` should return `"A"`
    *   `26` should return `"Z"`
    *   `27` should return `"AA"`
    *   `702` should return `"ZZ"`
    *   And other representative cases.

**4. `dmc_colors` Module Tests:**

*   Verify that `dmc_colors::get_dmc_colors()` correctly loads and parses the CSV data into the expected `Color` struct format, including `r`, `g`, `b`, and `hex` components.
*   (Optional): Test the `DeltaE` color difference calculation for a few known color pairs to ensure the color matching logic is mathematically sound.

---

### II. Performance Benchmarks (`criterion` crate)

Performance benchmarks will use the `criterion` crate to measure the execution time of critical functions, allowing for quantitative assessment of optimization efforts.

**Setup:**
Add `criterion` as a `dev-dependency` in `Cargo.toml`:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

Create a `benches` directory in your project root and a benchmark file (e.g., `benches/my_benchmark.rs`).

**1. `generate_gem_art` Benchmarks:**

*   **Image Size Impact:**
    *   Benchmark the function with different input image resolutions (e.g., small: 100x100px, medium: 500x500px, large: 1000x1000px, very large: 2000x2000px or more).
    *   This will help understand how processing time scales with the number of pixels.
*   **Color Count Impact:**
    *   Benchmark with a fixed input image size but varying numbers of selected DMC colors (e.g., 10, 100, 500, all available colors).
    *   This will reveal the performance impact of the color matching algorithm based on the size of the color palette.
*   **Fit vs. Crop Performance:**
    *   Benchmark the performance difference between `ImageFitOption::Fit` and `ImageFitOption::Crop` for a given image and canvas size to identify if one method is significantly more expensive.

**2. `generate_text_image` Benchmarks:**

*   **Gem Count Impact:**
    *   Benchmark the function with varying numbers of `GemCount` entries (e.g., 10, 50, 200, 500).
    *   This will show how the legend generation time scales with the complexity of the legend.

---

### III. Execution

*   **Run Unit Tests:** `cargo test`
*   **Run Benchmarks:** `cargo bench` (HTML reports will be generated in `target/criterion/report/index.html`)

This structured approach will provide clear metrics for performance improvements and ensure that any optimizations do not introduce regressions in functionality.