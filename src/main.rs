use comrak::{markdown_to_html, ComrakOptions};
use std::fs;

fn main() {
    println!("Opening README \n");

    let file_path = "README.md".to_string();
    let contents = fs::read_to_string(file_path).expect("Couldn't read file");

    let markdown = markdown_to_html(&contents, &ComrakOptions::default());

    println!("Markdown parsed into HTML \n");
    println!("{markdown}");
}
