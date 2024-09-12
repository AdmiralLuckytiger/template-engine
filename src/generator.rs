use std::collections::HashMap;
use crate::parser::{Conditional, ExpressionData, OperationType, ContentType, TagType, ConditionData};

/// Generates HTML code for a template var token
pub fn generate_html_template_var(content: ExpressionData, context: HashMap<String,Vec<String>>) -> String {
    let mut html = String::new();

    if let Some(h) = content.head {
        html.push_str(&h);
    }

    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val[0]);
    }

    if let Some(t) = content.tail {
        html.push_str(&t);
    }

    html
}

/// Generates HTML code for a if or for tag tokens
pub fn generate_html_tag(content: Conditional, context: HashMap<String,Vec<String>>) -> String {
    let mut html = String::new();

    match content.condition.operation {
        OperationType::Equal => {
            let right_operand: Vec<&str> = content.condition.right_operand.split(" ").collect();

            let left_operand: &Vec<String> = match context.get(&content.condition.left_operand) {
                Some(v)  => v,
                None    => return " ".to_string(),
            };

            if right_operand == *left_operand {
                match *content.expression {
                    ContentType::Literal(text) => html.push_str(&text),
                    ContentType::Tag(tag) => {
                        match tag {
                            TagType::IfTag(data) => {
                                html.push_str(&generate_html_tag(*data, context))
                            },
                            TagType::ForTag(data) => {
                                html.push_str(&generate_html_tag(*data, context))
                            }, 
                        }
                    },
                    ContentType::TemplateVariable(data) => html.push_str(&generate_html_template_var(data, context)),
                    ContentType::Unrecognized => html.push_str(""),
                }
            }
        },
        OperationType::In => {
            let right_operand: &Vec<String> = match context.get(&content.condition.right_operand) {
                Some(v)  => v,
                None    => return " ".to_string(),
            };

            for element in right_operand {
                match *content.expression {
                    ContentType::Literal(ref text) => {
                        html.push_str(&text);
                    },
                    ContentType::TemplateVariable(ref data) => {
                        if let Some(h) = &data.head.clone() {
                            html.push_str(&h);
                        }

                        html.push_str(&element);

                        if let Some(t) = &data.tail.clone() {
                            html.push_str(&t);
                        }
                    },
                    _ => {},
                }
                html.push_str("\n");
            }
        },
        OperationType::Nosoported(e) => {
            return e
        },
    }

    html
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::parser::get_conditional_data;

    use super::*;

    #[test]
    fn check_literals() {
        let mut context: HashMap<String,Vec<String>> = HashMap::new();

        context.insert("name".to_string(), vec!["Bob".to_string()]);
        context.insert("city".to_string(), vec!["Boston".to_string()]);

        assert_eq!(generate_html_template_var(ExpressionData { head: None, variable: "name".to_string(), tail: None,}, context), "Bob".to_string())
    }

    #[test]
    fn check_if_tag() {
        let mut context: HashMap<String,Vec<String>> = HashMap::new();

        context.insert("name".to_string(), vec!["Bob".to_string()]);
        context.insert("city".to_string(), vec!["Boston".to_string()]);

        assert_eq!(
            get_conditional_data("{% if name = Bob %} <h1> hello Bob </h1> {% endif %}").expect("Input for test"),
            Conditional {
                condition: ConditionData {
                    left_operand: "name".to_string(),
                    operation: OperationType::Equal,
                    right_operand: "Bob".to_string(),
                },
                expression: Box::new(ContentType::Literal("<h1> hello Bob </h1>".to_string()))
            }
        );

        assert_eq!(generate_html_tag(get_conditional_data("{% if name = Bob %} <h1> hello Bob </h1> {% endif %}").expect("Input for test"),context), "<h1> hello Bob </h1>".to_string())
    }

    #[test]
    fn check_if_tag_var() {
        let mut context: HashMap<String,Vec<String>> = HashMap::new();

        context.insert("name".to_string(), vec!["Bob".to_string()]);
        context.insert("city".to_string(), vec!["Boston".to_string()]);

        assert_eq!(
            get_conditional_data("{% if name = Bob %} <h1> hello {{ name }} </h1> {% endif %}").expect("Input for test"),
            Conditional {
                condition: ConditionData {
                    left_operand: "name".to_string(),
                    operation: OperationType::Equal,
                    right_operand: "Bob".to_string(),
                },
                expression: Box::new(ContentType::TemplateVariable(ExpressionData { head: Some("<h1> hello ".to_string()), variable: "name".to_string(), tail: Some(" </h1>".to_string()) }))
            }
        );

        assert_eq!(generate_html_tag(get_conditional_data("{% if name = Bob %} <h1> hello Bob </h1> {% endif %}").expect("Input for test"),context), "<h1> hello Bob </h1>".to_string())
    }

    #[test]
    fn check_for_tag_one() {
        let mut context: HashMap<String,Vec<String>> = HashMap::new();

        context.insert("name".to_string(), vec!["Bob".to_string(), "Lisa".to_string()]);
        context.insert("city".to_string(), vec!["Boston".to_string()]);

        assert_eq!(generate_html_tag(get_conditional_data("{% for costumer in name %} <li> {{customer}} </li> {% endfor %}").expect("Hardcoded input"),context), "<li> Bob </li>\n<li> Lisa </li>\n".to_string())
    }
}
