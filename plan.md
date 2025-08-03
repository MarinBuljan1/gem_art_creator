**Goal**
To convert an image into gem art using leftover coloured gems.

**Process**
To achieve this goal, the website will accept an image and allow someone to input all the different colours of the gems they have avaiable. After clicking the button to run, the website will process the image by converting it to the correct size and converting each pixel to the closest available gem. The output will be a downloadable image with a preview, of the new picture.

**Page Layout**
Contains an input for an image.
Add or Delete different colours.

**Conversion**
We will be converting the image into an A4 size with 3cm margins on each side.
Each pixel will be 2.7mm in width and height.
The image will keep its aspect ratio, doing the equivalent of "object-fit: contain" in CSS. Therefore we should not lose any of the picture. If there are extra margins, we treat those as margins.
Each pixel will then be converted to the closest gem colour selected.

**Steps**
1.  **Create the user interface:**
    *   An input field to accept an image file.
    *   A section to dynamically add and remove available gem colors.
    *   A "Generate" or "Run" button to start the conversion process.
    *   A preview area to display the resulting gem art image.
    *   A button to download the final image.

2.  **Implement the image conversion logic:**
    *   When an image is uploaded, read it into memory.
    *   Calculate the target dimensions. The image will be resized to fit within a 150mm x 237mm area (A4 with 3cm margins), while maintaining its original aspect ratio.
    *   Determine the number of "pixels" (gems) that will fit within these dimensions, given that each gem is 2.7mm x 2.7mm.
    *   For each pixel in the source image, find the closest color match from the user-defined list of gem colors.
    *   Create a new image where each "pixel" is replaced with the corresponding closest gem color.

3.  **Display and download the output:**
    *   Render the newly generated image in the on-screen preview area.
    *   Provide a mechanism for the user to download the generated image as a file.