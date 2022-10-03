# Rust Markdown Playground

Experimenting with **Markdown** and **[MDX](https://mdxjs.com/)** using Rust.

## Getting Started

1. Clone repo.
1. Run `cargo run` (or `CTRL+SHIFT+B` to find the run task)

> This project requires you to have Rust installed in your development environment.

## Examples

### Parse Markdown File to HTML

```rust
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
```

## References

- [comrak](https://github.com/kivikakk/comrak)
- [MDX JS](https://mdxjs.com/)
