# 🦀 `rust-devicons`

A Rust library inspired by [vim-devicons](https://github.com/ryanoasis/vim-devicons), that provides filetype glyphs (icons) for a wide range of common file formats.

<img width="2197" alt="rust-devicons" src="https://github.com/user-attachments/assets/f2beb589-f97d-4afd-9262-483f59bc2ad8">

## Features

- 🦞 **Icon Retrieval**: Get file or directory icons based on file name/extension.
- 🦞 **Icon Color**: Retrieve the color associated with the icon depending on the specified theme.
- 🦞 **Filetype Support**: Supports a wide range of filetypes and filename conventions (dockerfile, makefile, etc.).
- 🦞 **Customizable Themes**: Supports both light and dark themes.

<img width="1443" alt="Screenshot 2024-09-21 at 16 54 16" src="https://github.com/user-attachments/assets/4e60cb61-2e02-4f21-965c-2b934a15575c">


## Installation

```sh
cargo add devicons
```

_**NOTE**: you'll need to use a [NerdFont](https://www.nerdfonts.com/) to properly display the icons._

## Usage

Here’s a simple example of how to use `devicons` to retrieve a file icon with the dark theme:

```rust
use std::path::Path;
use devicons::{File, Theme, icon_for_file};

fn main() {
    // getting the icon from a path
    let path = Path::new("example.txt");
    let icon = icon_for_file(path, Some(Theme::Dark));

    // getting the icon from a str
    let icon = icon_for_file("example.txt", Some(Theme::Dark));

    println!("File: {}", path.to_string_lossy());
    println!("Icon: {}", icon.icon);
    println!("Color: {}", icon.color);
}
```

### Running the Examples

You can find more usage examples in the `examples` directory. To run them, use:

```bash
cargo run --example <example_name>
```

## License

This project is licensed under the [UNLICENSE license](LICENSE).

