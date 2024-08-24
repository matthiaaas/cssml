use crate::parser::ASTNode;

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }

    pub fn generate_html(&self, node: ASTNode) -> String {
        match node {
            ASTNode::Document(nodes) => {
                let mut html = String::new();
                for node in nodes {
                    html.push_str(&self.generate_html(node));
                }
                html
            }
            ASTNode::Element {
                tag,
                id,
                classes,
                content,
                children,
            } => {
                let mut html = format!("<{}", tag);
                if let Some(id) = id {
                    html.push_str(&format!(" id=\"{}\"", id));
                }
                if !classes.is_empty() {
                    html.push_str(&format!(" class=\"{}\"", classes.join(" ")));
                }
                html.push_str(">");
                if let Some(content) = content {
                    html.push_str(&content);
                }
                for child in children {
                    html.push_str(&self.generate_html(child));
                }
                html.push_str(&format!("</{}>", tag));
                html
            }
        }
    }
}
