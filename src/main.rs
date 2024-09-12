use std::{collections::HashMap, io, io::BufRead};

use template_engine::{
    generator::{generate_html_tag, generate_html_template_var}, parser::{get_content_type, ContentType, TagType}
    };


fn main() -> () {
    let mut context: HashMap<String,Vec<String>> = HashMap::new();

    context.insert("name".to_string(), vec!["Bob".to_string()]);
    context.insert("city".to_string(), vec!["Boston".to_string()]);

    for line in io::stdin().lock().lines() {
        match get_content_type(&line.unwrap().clone()) {
            ContentType::TemplateVariable(content) => {
                let html = generate_html_template_var(content, context.clone());
                println!("{}", html);
            },
            ContentType::Literal(text) => println!("{}", text),
            ContentType::Tag(TagType::ForTag(content)) => {
                let html = generate_html_tag(*content, context.clone());
                println!("{}", html);
            },
            ContentType::Tag(TagType::IfTag(content)) => {
                let html = generate_html_tag(*content, context.clone());
                println!("{}", html);
            },
            ContentType::Unrecognized => println!("Unrecognized input"),
        }
    }
}