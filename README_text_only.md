# bitmap_type_tracer


`bitmap_type_tracer` is a utility tool for generating images using a provided font bitmap and a set of configuration parameters.  

## Features
- Generate images from a font bitmap using a custom sequence and text.
- Load and save font configurations for reuse.
- Customize margins and threshold for more precise image generation.

## Getting Started
Ensure you have the `image` and `serde_derive` crates as dependencies in your project.

### Font Source
The fonts used in this project are sourced from [ianhan/BitmapFonts](https://github.com/ianhan/BitmapFonts/tree/main) repository.

### How to Run

#### Specifying Arguments Directly:

```bash
bitmap_type_tracer <path_to_font_image> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]
```

example:
```bash
bitmap_type_tracer "examples/4138906397_0dc616813b_o.png" " !\"    '()*+,-.\\0123456789:; = ? ABCDEFGHIJKLMNOPQRSTUVWXYZ " "Bitmap Type Tracer" 10 --threshold 1 --save-json
```

#### Using Font Configuration:
For this method, you will need to have a font configuration file saved. You can do this by running the command with the `--save-json` flag when you specified all the arguments.
```bash
bitmap_type_tracer <path_to_font_image> <text>
```

example:
```bash
bitmap_type_tracer "examples/4138906397_0dc616813b_o.png" "Bitmap Type Trace"
```

This command will generate an image using a previously saved font configuration for the provided font image and the specified text.


**Arguments:**

- `<path_to_font_image>`: Path to the font image (bitmap) you want to use.

- `<sequence>`: A sequence of characters as they appear in the font bitmap.

- `<text>`: The text you want to generate as an image.

- `<chars_per_row>`: Number of characters in a row in the font bitmap.

**Optional Flags:**

- `--top VALUE`: Specify top margin. Default is 0.

- `--bottom VALUE`: Specify bottom margin. Default is 0.

- `--left VALUE`: Specify left margin. Default is 0.

- `--right VALUE`: Specify right margin. Default is 0.

- `--threshold VALUE`: Specify the threshold for color comparisons. Default is 0.

- `--save-json`: Save the provided configuration as a JSON file, making it easier to reuse in the future.

## Modules

- `main.rs`: The main driver of the application, handling command line arguments and invoking image generation.

- `font_config.rs`: Handles the loading and saving of font configurations.

- `image_processing.rs`: Contains the core image processing functions. (This module was mentioned but its content was not provided in the given code.)


## Contributions

Feel free to contribute by opening issues or pull requests. All feedback is welcome!
