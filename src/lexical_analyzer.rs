/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char;

    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char);

    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool;
}


/// A concrete implementation of the lexical analyzer.
pub struct LolcodeLexicalAnalyzer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>,
    pub hai: Vec<String>,
    pub kbye: Vec<String>,
    pub obtw: Vec<String>,
    pub tldr: Vec<String>,
    pub maek: Vec<String>,
    pub oic: Vec<String>,
    pub gimmeh: Vec<String>,
    pub mkay: Vec<String>,
    pub head: Vec<String>,
    pub title: Vec<String>,
    pub paragraf: Vec<String>,
    pub bold: Vec<String>,
    pub italics: Vec<String>,
    pub list: Vec<String>,
    pub item: Vec<String>,
    pub newline: Vec<String>,
    pub linx: Vec<String>,
    pub ihaz: Vec<String>,
    pub itiz: Vec<String>,
    pub lemmesee: Vec<String>,
    pub var_name: Vec<String>,
    pub var_value: Vec<String>,
    pub text: Vec<String>,
    pub address: Vec<String>,
}

impl LolcodeLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            hai: vec!["#hai".into()],
            kbye: vec!["#kbye".into()],
            obtw: vec!["#obtw".into()],
            tldr: vec!["#tldr".into()],
            maek: vec!["#maek".into()],
            oic: vec!["#oic".into()],
            gimmeh: vec!["#gimmeh".into()],
            mkay: vec!["#mkay".into()],
            head: vec!["head".into()],
            title: vec!["title".into()],
            paragraf: vec!["paragraf".into()],
            bold: vec!["bold".into()],
            italics: vec!["italics".into()],
            list: vec!["list".into()],
            item: vec!["item".into()],
            newline: vec!["newline".into()],
            linx: vec!["linx".into()],
            ihaz: vec!["#ihas".into()],
            itiz: vec!["#itiz".into()],
            lemmesee: vec!["#lemmesee".into()],
            var_name: Vec::new(),                                                                                   
            var_value: Vec::new(),                                                                                  
            text: Vec::new(),                                                                                       
            address: Vec::new(),
        }
    }

    /// Check if a character is an allowed.
    fn is_allowed_char(c: char) -> bool {
        c.is_ascii_alphanumeric()
            || matches!(c, ',' | '.' | '"' | '\'' | ':' | '?' | '!' | '%' | '/' | '\t' | '\n' | ' ')
    }

    /// VARNAME ::= any single word (A–Z, a–z, no spaces)
    pub fn is_var_name(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic())
    }

    /// VARVALUE ::= allowed text characters (no spaces)
    pub fn is_var_value(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| Self::is_allowed_char(c) && c != ' ')
    }

    /// TEXT ::= plain text (allowed characters including spaces) *empty allowed
    pub fn is_text(s: &str) -> bool {
        s.chars().all(|c| Self::is_allowed_char(c))
    }

    /// ADDRESS ::= text without spaces
    pub fn is_address(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| Self::is_allowed_char(c) && c != ' ')
    }

    pub fn tokenize(&mut self) {
        loop {
            let c = self.get_char();
            if c == '\0' {
                break;
            }
            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    self.tokens.push(std::mem::take(&mut self.current_build));
                }
            } else {
                self.add_char(c);
            }
        }
        if !self.current_build.is_empty() {
            self.tokens.push(std::mem::take(&mut self.current_build));
        }
        self.tokens.reverse();
    }
}

impl LexicalAnalyzer for LolcodeLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        let lower = s.to_ascii_lowercase();
        self.hai.iter().any(|t| t == &lower)
            || self.kbye.iter().any(|t| t == &lower)
            || self.obtw.iter().any(|t| t == &lower)
            || self.tldr.iter().any(|t| t == &lower)
            || self.maek.iter().any(|t| t == &lower)
            || self.oic.iter().any(|t| t == &lower)
            || self.gimmeh.iter().any(|t| t == &lower)
            || self.mkay.iter().any(|t| t == &lower)
            || self.head.iter().any(|t| t == &lower)
            || self.title.iter().any(|t| t == &lower)
            || self.paragraf.iter().any(|t| t == &lower)
            || self.bold.iter().any(|t| t == &lower)
            || self.italics.iter().any(|t| t == &lower)
            || self.list.iter().any(|t| t == &lower)
            || self.item.iter().any(|t| t == &lower)
            || self.newline.iter().any(|t| t == &lower)
            || self.linx.iter().any(|t| t == &lower)
            || self.ihaz.iter().any(|t| t == &lower)
            || self.itiz.iter().any(|t| t == &lower)
            || self.lemmesee.iter().any(|t| t == &lower)
            || Self::is_var_name(&lower)                                                                                 
            || Self::is_var_value(&lower)                                                                                
            || Self::is_text(&lower)                                                                                     
            || Self::is_address(&lower)
    }
}