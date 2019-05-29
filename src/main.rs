#[macro_use]
extern crate lazy_static;
extern crate comrak;
extern crate regex;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod ast_nodes_iter;
mod cus_error;

use crate::cus_error::ConvertError;
use crate::cus_error::RegexError;
use crate::cus_error::SnippitError;

static VARIBLEMARKER: &'static str = "$$";

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn make_snippits(string_to_read: &str) -> Result<HashMap<String, String>, SnippitError> {
    lazy_static! {
        static ref KEY: Regex = Regex::new(r".*:").unwrap();
        static ref VALUE: Regex = Regex::new(r" .*").unwrap();
    }
    let mut snip = HashMap::new();
    let mut lines = string_to_read.lines();

    while let Some(x) = lines.next() {
        let buf = String::from(x);
        if !buf.contains("******") {
            let key_val = KEY.find(&buf).ok_or(RegexError);
            let value_val = VALUE.find(&buf).ok_or(RegexError);
            let key_str = buf.substring(key_val.clone()?.start(), key_val.clone()?.end() - 1);
            let value_str = buf.substring(value_val.clone()?.start() + 1, value_val.clone()?.end());
            snip.insert(key_str, value_str);
        }
    }
    Ok(snip)
}

fn snippit_replace<'a>(root: &'a AstNode<'a>, str_search: &str, string_replace: &str) {
    ast_nodes_iter::iter_nodes(root, &|node| match *(&mut node.data.borrow_mut().value) {
        NodeValue::Text(ref mut text) => {
            let orig = std::mem::replace(text, vec![]);
            *text = String::from_utf8(orig)
                .unwrap() //deal With later
                .replace(&str_search, string_replace)
                .as_bytes()
                .to_vec();
        }
        _ => (),
    })
}

fn snippit_replacer_to_markdown(
    md: &str,
    snipits: HashMap<String, String>,
) -> Result<String, ConvertError> {
    let arena = Arena::new();
    let mut html = vec![];

    let root = parse_document(&arena, md, &ComrakOptions::default());

    for (key, value) in &snipits {
        snippit_replace(
            root,
            &(VARIBLEMARKER.to_string() + &key + VARIBLEMARKER),
            value,
        )
    }
    format_html(root, &ComrakOptions::default(), &mut html)?;

    Ok(String::from_utf8(html)?)
}

fn main() {
    let mut buf = String::new();
    let f = File::open("testtemplate.md").unwrap();
    let mut f = BufReader::new(f);
    while !buf.contains("******") {
        f.read_line(&mut buf).unwrap();
    }
    let snippits = make_snippits(&buf);
    buf.clear();
    f.read_to_string(&mut buf).unwrap();

    match snippits {
        Err(error) => println!("Error: {}", error),
        Ok(x) => {
            let html_output = snippit_replacer_to_markdown(&buf, x);
            match html_output {
                Err(error) => println!("Error: {}", error),
                Ok(y) => println!("{}", y),
            }
        }
    }
}
