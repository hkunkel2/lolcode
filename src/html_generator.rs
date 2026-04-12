use crate::syntax_analyzer::Node;

// ===================== HtmlGenerator =====================

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generates an HTML string from the resolved syntax tree.
    pub fn generate(&self, tree: &[Node]) -> String {
        let mut tokens = Vec::new();
        self.tree_to_tokens(tree, &mut tokens);
        self.tokens_to_html(&tokens)
    }

    /// Recursively walks the Node tree and collects all string tokens into a flat list.
    fn tree_to_tokens(&self, nodes: &[Node], out: &mut Vec<String>) {
        for node in nodes {
            match node {
                Node::Str(s)      => out.push(s.clone()),
                Node::List(items) => self.tree_to_tokens(items, out),
            }
        }
    }

    /// Walks the token list linearly, using a closing-tag stack to produce HTML.
    fn tokens_to_html(&self, tokens: &[String]) -> String {
        let mut out = String::new();
        out.push_str("<!DOCTYPE html>\n<html>\n");

        let mut i = 0;
        let mut stack: Vec<String> = Vec::new();
        let mut body_open = false;

        while i < tokens.len() {
            let lc = tokens[i].to_ascii_lowercase();

            match lc.as_str() {
                // program delimiters — no HTML output
                "#hai" | "#kbye" => {}

                // comment block — collect text and outpots as HTML comment
                "#obtw" => {
                    i += 1;
                    let mut comment_words = Vec::new();
                    while i < tokens.len() && tokens[i].to_ascii_lowercase() != "#tldr" {
                        comment_words.push(tokens[i].as_str());
                        i += 1;
                    }
                    out.push_str(&format!("<!-- {} -->\n", comment_words.join(" ")));
                    // i now points at #tldr; outer i += 1 steps past it
                }

                "#tldr" => {} // end-of-comment marker, consumed by #obtw loop above

                // close the current block by popping the stack
                "#mkay" | "#oic" => {
                    if let Some(closing) = stack.pop() {
                        out.push_str(&closing);
                        // after </head>, open <body>
                        if closing.starts_with("</head>") && !body_open {
                            out.push_str("<body>\n");
                            body_open = true;
                        }
                    }
                }

                // #maek <keyword> ... #mkay
                "#maek" => {
                    i += 1;
                    if i < tokens.len() {
                        match tokens[i].to_ascii_lowercase().as_str() {
                            "head" => {
                                out.push_str("<head>");
                                stack.push("</head>\n".to_string());
                            }
                            "paragraf" => {
                                if !body_open { out.push_str("<body>\n"); body_open = true; }
                                out.push_str("<p>");
                                stack.push("</p>\n".to_string());
                            }
                            "list" => {
                                if !body_open { out.push_str("<body>\n"); body_open = true; }
                                out.push_str("<ul>\n");
                                stack.push("</ul>\n".to_string());
                            }
                            _ => { stack.push(String::new()); }
                        }
                    }
                }

                // #gimmeh <keyword> ... #oic
                "#gimmeh" => {
                    i += 1;
                    if i < tokens.len() {
                        match tokens[i].to_ascii_lowercase().as_str() {
                            "title"   => { out.push_str("<title>");  stack.push("</title>".to_string()); }
                            "item"    => { out.push_str("<li>");     stack.push("</li>\n".to_string()); }
                            "bold"    => { out.push_str("<b>");      stack.push("</b>".to_string()); }
                            "italics" => { out.push_str("<i>");      stack.push("</i>".to_string()); }
                            "newline" => {
                                out.push_str("<br>\n");
                                stack.push(String::new()); // #oic pops this harmlessly
                            }
                            "linx" => {
                                i += 1; // URL token
                                let url = if i < tokens.len() { tokens[i].as_str() } else { "" };
                                out.push_str(&format!("<a href=\"{}\">{}</a>", url, url));
                                i += 1; // consume the #oic — no stack push needed
                            }
                            _ => { stack.push(String::new()); }
                        }
                    }
                }

                // plain text token
                _ => {
                    out.push_str(&tokens[i]);
                    out.push(' ');
                }
            }

            i += 1;
        }

        if body_open {
            out.push_str("</body>\n");
        } else {
            out.push_str("<body>\n</body>\n");
        }
        out.push_str("</html>\n");
        out
    }
}
