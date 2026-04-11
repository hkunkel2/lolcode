use crate::lexical_analyzer::{LolcodeLexicalAnalyzer, LexicalAnalyzer};
use crate::Compiler;

/// Str  = a string (token)
/// List = a list of strings and/or other lists
#[derive(Debug)]
pub enum Node {
    Str(String),
    List(Vec<Node>),
}

/// ===================== SyntaxAnalyzer Trait =====================
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_link(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

/// ===================== Syntax Analyzer =====================
pub struct LolcodeCompiler {
    lexer: LolcodeLexicalAnalyzer,
    current_tok: String,
    pub tree: Vec<Node>,
    stack: Vec<Vec<Node>>,
    /// Holds a pre-consumed dispatch token (#maek / #gimmeh) so it lands
    /// as the first string inside the sub-list that owns it.
    pending: Option<String>,
}

impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            lexer: LolcodeLexicalAnalyzer::new(""),
            current_tok: String::new(),
            tree: Vec::new(),
            stack: Vec::new(),
            pending: None,
        }
    }

    fn open_list(&mut self) {
        self.stack.push(Vec::new());
        // If a dispatch token (#maek / #gimmeh) was saved, make it the first
        // string in this new scope.
        if let Some(tok) = self.pending.take() {
            self.add_str(tok);
        }
    }

    fn add_str(&mut self, s: String) {
        if let Some(top) = self.stack.last_mut() {
            top.push(Node::Str(s));
        }
    }

    fn close_list(&mut self) {
        if let Some(finished) = self.stack.pop() {
            let node = Node::List(finished);
            if let Some(parent) = self.stack.last_mut() {
                parent.push(node);
            } else {
                self.tree.push(node);
            }
        }
    }

    fn start(&mut self) {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate;
        } else if !candidate.is_empty() {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        } else {
            eprintln!("Error: the provided source is empty.");
            std::process::exit(1);
        }
    }

    /// is_hai() - token == #hai
    #[inline] fn is_hai(&self, s: &str)      -> bool { self.lexer.hai.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_kbye() - token == #kbye
    #[inline] fn is_kbye(&self, s: &str)     -> bool { self.lexer.kbye.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_obtw() - token == #obtw
    #[inline] fn is_obtw(&self, s: &str)     -> bool { self.lexer.obtw.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_tldr() - token == #tldr
    #[inline] fn is_tldr(&self, s: &str)     -> bool { self.lexer.tldr.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_maek() - token == #maek
    #[inline] fn is_maek(&self, s: &str)     -> bool { self.lexer.maek.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_oic() - token == #oic
    #[inline] fn is_oic(&self, s: &str)      -> bool { self.lexer.oic.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_gimmeh() - token == #gimmeh
    #[inline] fn is_gimmeh(&self, s: &str)   -> bool { self.lexer.gimmeh.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_mkay() - token == #mkay
    #[inline] fn is_mkay(&self, s: &str)     -> bool { self.lexer.mkay.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_head() - token == #head
    #[inline] fn is_head(&self, s: &str)     -> bool { self.lexer.head.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_title() - token == #is_title
    #[inline] fn is_title(&self, s: &str)    -> bool { self.lexer.title.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_paragraf() - token == paragraf
    #[inline] fn is_paragraf(&self, s: &str) -> bool { self.lexer.paragraf.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_bold() - token == bold
    #[inline] fn is_bold(&self, s: &str)     -> bool { self.lexer.bold.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_italics() - token == italics
    #[inline] fn is_italics(&self, s: &str)  -> bool { self.lexer.italics.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_list() - token == list
    #[inline] fn is_list(&self, s: &str)     -> bool { self.lexer.list.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_item() - token == item
    #[inline] fn is_item(&self, s: &str)     -> bool { self.lexer.item.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_newline() - token == newline
    #[inline] fn is_newline(&self, s: &str)  -> bool { self.lexer.newline.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_linx() - token == linx
    #[inline] fn is_linx(&self, s: &str)     -> bool { self.lexer.linx.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_ihaz() - token == #ihaz
    #[inline] fn is_ihaz(&self, s: &str)     -> bool { self.lexer.ihaz.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_itiz() - token == #itiz
    #[inline] fn is_itiz(&self, s: &str)     -> bool { self.lexer.itiz.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_lemmesee() - token == #lemmesee
    #[inline] fn is_lemmesee(&self, s: &str) -> bool { self.lexer.lemmesee.iter().any(|t| t == &s.to_ascii_lowercase()) }
    /// is_var_name() — any single word (A–Z, a–z, no spaces)
    #[inline] fn is_var_name(&self, s: &str)  -> bool { LolcodeLexicalAnalyzer::is_var_name(s) }
    /// is_var_value() — VARVALUE ::= allowed text characters
    #[inline] fn is_var_value(&self, s: &str) -> bool { LolcodeLexicalAnalyzer::is_var_value(s) }
    /// is_text() —  plain text (letters, digits, punctuation, spaces)
    #[inline] fn is_text(&self, s: &str)      -> bool { LolcodeLexicalAnalyzer::is_text(s) }
    /// is_address() — text without spaces
    #[inline] fn is_address(&self, s: &str)   -> bool { LolcodeLexicalAnalyzer::is_address(s) }

}

/// ===================== Compiler Trait Impl =====================
impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = LolcodeLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            candidate
        } else if candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    fn parse(&mut self) {
        self.parse_lolcode();
    }

    fn current_token(&self) -> String {
        self.current_tok.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_tok = tok;
    }
}

//
// ===================== SyntaxAnalyzer Trait Impl =====================
//

impl SyntaxAnalyzer for LolcodeCompiler {

    /// <lolcode> ::= HAI <comments> <head> <body> KBYE
    fn parse_lolcode(&mut self) {
        self.open_list();
        if self.is_hai(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#hai' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_comment();

        // multiple tokens can be expected after #maek need to account for head, paragraf and list
        if self.is_maek(&self.current_tok) {
            self.pending = Some(self.current_tok.clone());
            self.next_token();
            if self.is_head(&self.current_tok) {
                self.parse_head();
            } else if self.is_paragraf(&self.current_tok) {
                self.parse_paragraph();
            } else if self.is_list(&self.current_tok) {
                self.parse_list();
            } else {
                eprintln!("Syntax error: '{}' was found when 'head', 'paragraf', or 'list' was expected after '#maek'.", self.current_tok);
                std::process::exit(1);
            }
        }

        self.parse_body();

        if self.is_kbye(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#kbye' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <head> ::= HEAD <title> MKAY
    /// Called with current_tok == "head" (parse_lolcode already consumed #maek).
    fn parse_head(&mut self) {
        self.open_list();
        if self.is_head(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'head' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_title();

        if self.is_mkay(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#mkay' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <title> ::= GIMMEH TITLE TEXT OIC
    fn parse_title(&mut self) {
        self.open_list();
        if self.is_gimmeh(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#gimmeh' was expected.", self.current_tok);
            std::process::exit(1);
        }

        if self.is_title(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'title' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_text();

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <comments> ::= <comment> <comments> | empty
    /// <comment>  ::= OBTW TEXT TLDR
    fn parse_comment(&mut self) {
        while self.is_obtw(&self.current_tok) {
            self.open_list();
            self.add_str(self.current_tok.clone());
            self.next_token();
            self.parse_text();
            if self.is_tldr(&self.current_tok) {
                self.add_str(self.current_tok.clone());
                self.next_token();
            } else {
                eprintln!("Syntax error: '{}' was found when '#tldr' was expected.", self.current_tok);
                std::process::exit(1);
            }
            self.close_list();
        }
    }

    /// <body> ::= <inner-body> <body> | empty
    /// <inner-body> ::= <paragraph>
    ///     | <comment>
    ///     | <bold>
    ///     | <italics>
    ///     | <list>
    ///     | <link>
    ///     | <newline>
    ///     | <variable-define>
    ///     | <variable-use>
    ///     | TEXT
    fn parse_body(&mut self) {
        while !self.is_kbye(&self.current_tok) && !self.current_tok.is_empty() {
            if self.is_obtw(&self.current_tok) {
                self.parse_comment();
            } else if self.is_ihaz(&self.current_tok) {
                self.parse_variable_define();
            } else if self.is_lemmesee(&self.current_tok) {
                self.parse_variable_use();
            } else if self.is_maek(&self.current_tok) {
                self.pending = Some(self.current_tok.clone());
                self.next_token();
                if self.is_paragraf(&self.current_tok) {
                    self.parse_paragraph();
                } else if self.is_list(&self.current_tok) {
                    self.parse_list();
                } else {
                    eprintln!("Syntax error: '{}' was found when 'paragraf' or 'list' was expected after '#maek'.", self.current_tok);
                    std::process::exit(1);
                }
            } else if self.is_gimmeh(&self.current_tok) {
                self.pending = Some(self.current_tok.clone());
                self.next_token();
                if self.is_bold(&self.current_tok) {
                    self.parse_bold();
                } else if self.is_italics(&self.current_tok) {
                    self.parse_italics();
                } else if self.is_linx(&self.current_tok) {
                    self.parse_link();
                } else if self.is_newline(&self.current_tok) {
                    self.parse_newline();
                } else {
                    eprintln!("Syntax error: '{}' was found after '#gimmeh' in body.", self.current_tok);
                    std::process::exit(1);
                }
            } else if self.is_text(&self.current_tok) {
                self.parse_text();
            } else {
                eprintln!("Syntax error: unexpected token '{}' in body.", self.current_tok);
                std::process::exit(1);
            }
        }
    }

    /// <paragraph> ::= PARAGRAF <variable-define> <inner-paragraph> MKAY
    /// Called with current_tok == "paragraf" (caller already consumed #maek).
    /// <inner-paragraph> ::= <inner-text> <inner-paragraph> | empty
    /// <inner-text> ::= <variable-use>
    ///     | <bold>
    ///     | <italics>
    ///     | <list>
    ///     | <link>
    ///     | <newline>
    ///     | TEXT
    ///     | empty
    fn parse_paragraph(&mut self) {
        self.open_list();
        if self.is_paragraf(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'paragraf' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_variable_define();
        self.parse_inner_paragraph();

        if self.is_mkay(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#mkay' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <inner-paragraph> ::= <inner-text> <inner-paragraph> | empty
    fn parse_inner_paragraph(&mut self) {
        while !self.is_mkay(&self.current_tok) && !self.current_tok.is_empty() {
            self.parse_inner_text();
        }
    }

    /// <inner-text> dispatcher — same as body but stops at #mkay
    fn parse_inner_text(&mut self) {
        if self.is_mkay(&self.current_tok) || self.current_tok.is_empty() {
            // empty
        } else if self.is_lemmesee(&self.current_tok) {
            self.parse_variable_use();
        } else if self.is_ihaz(&self.current_tok) {
            self.parse_variable_define();
        } else if self.is_obtw(&self.current_tok) {
            self.parse_comment();
        } else if self.is_maek(&self.current_tok) {
            self.pending = Some(self.current_tok.clone());
            self.next_token();
            if self.is_list(&self.current_tok) {
                self.parse_list();
            } else {
                eprintln!("Syntax error: '{}' was found when 'list' was expected after '#maek' inside paragraph.", self.current_tok);
                std::process::exit(1);
            }
        } else if self.is_gimmeh(&self.current_tok) {
            self.pending = Some(self.current_tok.clone());
            self.next_token();
            if self.is_bold(&self.current_tok) {
                self.parse_bold();
            } else if self.is_italics(&self.current_tok) {
                self.parse_italics();
            } else if self.is_linx(&self.current_tok) {
                self.parse_link();
            } else if self.is_newline(&self.current_tok) {
                self.parse_newline();
            } else {
                eprintln!("Syntax error: '{}' was found after '#gimmeh' inside paragraph.", self.current_tok);
                std::process::exit(1);
            }
        } else if self.is_text(&self.current_tok) {
            self.parse_text();
        } else {
            eprintln!("Syntax error: unexpected token '{}' inside paragraph.", self.current_tok);
            std::process::exit(1);
        }
    }

    /// <variable-define> ::= IHAZ VARNAME ITIZ VARVALUE MKAY | empty
    fn parse_variable_define(&mut self) {
        if !self.is_ihaz(&self.current_tok) {
            return; // empty
        }

        self.open_list();
        self.add_str(self.current_tok.clone()); // #ihaz
        self.next_token();

        // VARNAME ::= any single word (A–Z, a–z, no spaces)
        if !self.is_var_name(&self.current_tok) {
            eprintln!("Syntax error: '{}' was found when a variable name (letters only) was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.add_str(self.current_tok.clone()); // VARNAME
        self.next_token();

        if self.is_itiz(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#itiz' was expected.", self.current_tok);
            std::process::exit(1);
        }

        // VARVALUE ::= allowed text characters
        if !self.is_var_value(&self.current_tok) {
            eprintln!("Syntax error: '{}' was found when a variable value was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.add_str(self.current_tok.clone()); // VARVALUE
        self.next_token();

        if self.is_mkay(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#mkay' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <variable-use> ::= LEMMESEE VARNAME OIC
    fn parse_variable_use(&mut self) {
        self.open_list();
        if self.is_lemmesee(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#lemmesee' was expected.", self.current_tok);
            std::process::exit(1);
        }

        // VARNAME ::= any single word (A–Z, a–z, no spaces)
        if !self.is_var_name(&self.current_tok) {
            eprintln!("Syntax error: '{}' was found when a variable name (letters only) was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.add_str(self.current_tok.clone()); // VARNAME
        self.next_token();

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <bold> ::= BOLD TEXT OIC
    /// Called with current_tok == "bold" (caller already consumed #gimmeh).
    fn parse_bold(&mut self) {
        self.open_list();
        if self.is_bold(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'bold' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_text();

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <italics> ::= ITALICS TEXT OIC
    /// Called with current_tok == "italics" (caller already consumed #gimmeh).
    fn parse_italics(&mut self) {
        self.open_list();
        if self.is_italics(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'italics' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_text();

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <list> ::= LIST <list-items> MKAY
    /// Called with current_tok == "list" (caller already consumed #maek).
    fn parse_list(&mut self) {
        self.open_list();
        if self.is_list(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'list' was expected.", self.current_tok);
            std::process::exit(1);
        }

        self.parse_list_items();

        if self.is_mkay(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#mkay' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <list-items> ::= <list-item> <list-items> | empty
    fn parse_list_items(&mut self) {
        while self.is_gimmeh(&self.current_tok) {
            self.parse_inner_list();
        }
    }

    /// <list-item> ::= GIMMEH ITEM <inner-list> OIC
    /// Called with current_tok == "#gimmeh" (NOT pre-consumed).
    /// <inner-list> ::= <bold> | <italics> | <link> | TEXT | <variable-use> | empty
    fn parse_inner_list(&mut self) {
        self.open_list();
        if self.is_gimmeh(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#gimmeh' was expected.", self.current_tok);
            std::process::exit(1);
        }

        if self.is_item(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'item' was expected.", self.current_tok);
            std::process::exit(1);
        }

        if self.is_gimmeh(&self.current_tok) {
            self.pending = Some(self.current_tok.clone());
            self.next_token();
            if self.is_bold(&self.current_tok) {
                self.parse_bold();
            } else if self.is_italics(&self.current_tok) {
                self.parse_italics();
            } else if self.is_linx(&self.current_tok) {
                self.parse_link();
            } else {
                eprintln!("Syntax error: '{}' was found after '#gimmeh' inside list item.", self.current_tok);
                std::process::exit(1);
            }
        } else if self.is_lemmesee(&self.current_tok) {
            self.parse_variable_use();
        } else if self.is_text(&self.current_tok) && !self.current_tok.is_empty() {
            self.parse_text();
        }
        // else empty — fall through to #oic

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <link> ::= LINX ADDRESS OIC
    /// Called with current_tok == "linx" (caller already consumed #gimmeh).
    fn parse_link(&mut self) {
        self.open_list();
        if self.is_linx(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'linx' was expected.", self.current_tok);
            std::process::exit(1);
        }

        // ADDRESS ::= text without spaces
        if !self.is_address(&self.current_tok) {
            eprintln!("Syntax error: '{}' was found when an address (no spaces) was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.add_str(self.current_tok.clone()); // ADDRESS
        self.next_token();

        if self.is_oic(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when '#oic' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// <newline> ::= NEWLINE
    /// Called with current_tok == "newline" (caller already consumed #gimmeh).
    fn parse_newline(&mut self) {
        self.open_list();
        if self.is_newline(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when 'newline' was expected.", self.current_tok);
            std::process::exit(1);
        }
        self.close_list();
    }

    /// TEXT ::= plain text (letters, digits, punctuation, spaces)
    /// Collects consecutive non-keyword text tokens.
    fn parse_text(&mut self) {
        while !self.current_tok.is_empty() && self.is_text(&self.current_tok) {
            self.add_str(self.current_tok.clone());
            self.next_token();
        }
    }
}
