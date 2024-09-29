use devicons::{icon_for_file, FileIcon, Theme};
use std::path::Path;

fn main() {
    // getting the icon from a path with a specified theme
    let path = Path::new("Cargo.toml");
    let icon_1 = icon_for_file(path, Some(Theme::Dark));

    // getting the icon from a string with a specified theme
    let icon_2 = icon_for_file("Cargo.toml", Some(Theme::Dark));

    // getting the icon from a path with the default theme
    let icon_3 = FileIcon::from(path);

    // directly getting an icon from a filename
    let icon_4 = FileIcon::from("Cargo.toml");

    println!("File: {}", path.to_string_lossy());
    println!("Icon: {} {}", icon_1.icon, icon_1.color);
    println!("Icon: {} {}", icon_2.icon, icon_2.color);
    println!("Icon: {} {}", icon_3.icon, icon_3.color);
    println!("Icon: {} {}", icon_4.icon, icon_4.color);
}
