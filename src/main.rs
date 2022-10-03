use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    println!("Hello, world!");

    assert_eq!(
        markdown_to_html("Hello, **世界**!", &ComrakOptions::default()),
        "<p>Hello, <strong>世界</strong>!</p>\n"
    );

    let markdown = markdown_to_html("Hello, **世界**!", &ComrakOptions::default());

    println!("{markdown}");
}
