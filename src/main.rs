use image::{ImageBuffer, Rgb};
use std::io::{self, Write};

const DEFAULT_COLOR: &str = "#87CEEB";

fn parse_hex_color(input: &str) -> Result<Rgb<u8>, String> {
    let trimmed = input.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);

    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "invalid hex color \"{input}\" (expected format like \"#RRGGBB\")"
        ));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Ok(Rgb([r, g, b]))
}

fn prompt_color() -> Rgb<u8> {
    print!("Enter hex color [{DEFAULT_COLOR}]: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let trimmed = input.trim();
    let hex = if trimmed.is_empty() { DEFAULT_COLOR } else { trimmed };

    parse_hex_color(hex).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    })
}

fn main() {
    let width = 500;
    let height = 500;

    let color = match std::env::args().nth(1) {
        Some(arg) => parse_hex_color(&arg).unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        }),
        None => prompt_color(),
    };

    let img = ImageBuffer::from_fn(width, height, |_x, _y| color);

    img.save("image.png").expect("Failed to save PNG");
}
