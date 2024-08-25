use std::fmt::Write;

#[derive(Debug)]
pub enum GenerationError {
    InvalidSelectorChain,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Document(Vec<ASTNode>),
    Selector {
        tag: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
        element: Option<String>,
        children: Vec<ASTNode>,
    },
    StyleRuleset {
        declarations: Vec<(String, String)>,
    },
}

impl ASTNode {
    pub fn to_html(&self) -> Result<String, GenerationError> {
        self.generate_html(&mut vec![])
    }

    fn generate_html(&self, selector_chain: &mut Vec<ASTNode>) -> Result<String, GenerationError> {
        match &self {
            ASTNode::Document(children) => {
                let mut html = String::new();
                for child in children {
                    write!(html, "{}", child.generate_html(selector_chain)?);
                }
                Ok(html)
            }
            ASTNode::Selector {
                tag,
                id,
                classes,
                element,
                children,
            } => {
                let is_generatable = tag.is_some() || id.is_some() || !classes.is_empty();
                let will_generate_element = element.is_some() && is_generatable;

                let mut html = String::new();

                let tag = tag.as_ref();
                let id = id.as_ref();
                let classes = classes;

                println!(
                    "{} #{} .{} {:?} {}",
                    tag.unwrap_or(&"".to_string()),
                    id.unwrap_or(&"".to_string()),
                    classes.join(" "),
                    selector_chain.len(),
                    selector_chain
                        .clone()
                        .into_iter()
                        .map(|node| match node {
                            ASTNode::Selector { tag, .. } => {
                                format!("{}", tag.unwrap_or("".to_string()),)
                            }
                            _ => "DONT KNOW".to_string(),
                        })
                        .reduce(|acc, x| format!("{}, {}", acc, x))
                        .unwrap_or("".to_string())
                );

                if will_generate_element {
                    write!(
                        html,
                        "<{}{}>{}",
                        tag.unwrap(),
                        self.generate_element_attributes(id, classes),
                        element.as_ref().unwrap_or(&"".to_string())
                    );
                }

                for child in children {
                    selector_chain.push(self.clone());
                    write!(html, "{}", child.generate_html(selector_chain)?);
                    selector_chain.pop();
                }

                if will_generate_element {
                    write!(html, "</{}>", tag.unwrap());
                }

                Ok(html)
            }
            ASTNode::StyleRuleset { declarations } => {
                let mut html = String::new();
                let mut rules = String::new();

                for (property, value) in declarations {
                    write!(rules, "{}: {};", property, value);
                }

                let css_selector = self.generate_css_selector(selector_chain)?;
                write!(html, "<style>{}{{{}}}</style>", css_selector, rules);

                Ok(html)
            }
            _ => Ok("Unimplemented".to_string()),
        }
    }

    fn generate_element_attributes(&self, id: Option<&String>, classes: &Vec<String>) -> String {
        let mut attributes = String::new();

        if let Some(id) = id {
            write!(attributes, " id=\"{}\"", id);
        }

        if !classes.is_empty() {
            write!(attributes, " class=\"{}\"", classes.join(" "));
        }

        attributes
    }

    fn generate_css_selector(
        &self,
        selector_chain: &mut Vec<ASTNode>,
    ) -> Result<String, GenerationError> {
        let mut selector = String::new();

        for node in selector_chain {
            match node {
                ASTNode::Selector {
                    tag, id, classes, ..
                } => {
                    let tag = tag.as_ref();
                    let id = id.as_ref();
                    let classes = classes;

                    if let Some(tag) = tag {
                        write!(selector, "{}", tag);
                    }

                    if let Some(id) = id {
                        write!(selector, "#{}", id);
                    }

                    for class in classes {
                        write!(selector, ".{}", class);
                    }

                    write!(selector, " ");
                }
                _ => Err(GenerationError::InvalidSelectorChain)?,
            }
        }

        Ok(selector)
    }
}
