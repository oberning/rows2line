extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn convert_to_line(s: &str) -> String {
    s.split(|c| c == '\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| match s.trim().parse::<f64>() {
            Ok(_ok) => format!("{}", s),
            Err(_e) => format!("'{}'", s),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let copied = ctx.get_contents().unwrap();
    let to_paste = convert_to_line(&copied);
    //println!("{}", split(content));
    ctx.set_contents(to_paste).unwrap();
}
