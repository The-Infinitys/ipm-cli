pub mod question;
use super::color;

pub fn color_str(s: String, hex: color::rgb) -> String {
    println!("{}", hex);
    return s;
}

mod tests {}
