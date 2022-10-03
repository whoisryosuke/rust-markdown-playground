use std::fs;
extern crate comrak;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};

fn main() {
    println!("Opening README \n");

    let file_path = "README.md".to_string();
    let contents = fs::read_to_string(file_path).expect("Couldn't read file");

    println!("Markdown parsed into AST \n");

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    // The "root" node, which we parse our markdown into
    let root = parse_document(&arena, &contents, &ComrakOptions::default());

    // Iterate through the nodes (and their children) recursively
    // We pass the node to the callback provided as the second function param
    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    // Call the iterate nodes function
    iter_nodes(root, &|node| {
        // Check the value of the node data
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::CodeBlock(ref mut block) => {
                let orig = std::mem::replace(&mut block.literal, vec![]);
                println!("Code Block: {}", String::from_utf8(orig).unwrap());
            }
            &mut NodeValue::Link(ref mut link) => {
                let orig = std::mem::replace(&mut link.url, vec![]);
                println!("Link: {}", String::from_utf8(orig).unwrap());
            }
            &mut NodeValue::Strong => {
                println!("Bold text: ");
            }
            // Got text?
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                println!("{}", String::from_utf8(orig).unwrap());
                // *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
            }
            _ => (),
        }
    });
}
