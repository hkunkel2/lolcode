use std::env;
use std::fs;

//
// ===================== Compiler Trait =====================
//

/// Compiler trait
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

//
// ===================== Lexical Analyzer =====================
//

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis.
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// A concrete implementation of the lexical analyzer.
pub struct SimpleLexicalAnalyzer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>,
    pub articles: Vec<String>,
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
    pub adverbs: Vec<String>,
    pub adjectives: Vec<String>,
}

impl SimpleLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            articles: vec!["a".into(), "teh".into()],
            verbs: vec!["lovez".into(), "hatez".into(), "ates".into()],
            nouns: vec!["dawg".into(), "kat".into(), "rat".into()],
            adverbs: vec!["accidently".into(), "quickly".into(), "secretly".into()],
            adjectives: vec!["fat".into(), "hungry".into(), "happy".into(), "mean".into()],
        }
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

impl LexicalAnalyzer for SimpleLexicalAnalyzer {
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
        self.articles.iter().any(|a| a == s)
            || self.nouns.iter().any(|n| n == s)
            || self.verbs.iter().any(|v| v == s)
            || self.adverbs.iter().any(|adv| adv == s)
            || self.adjectives.iter().any(|adj| adj == s)
    }
}

//
// ===================== Syntax Analyzer =====================
//

/// <sentence>    ::= <noun_phrase> [<adverb>] <verb> <noun_phrase>
/// <noun_phrase> ::= <article> <adjective> <noun>
pub struct LolspeakCompiler {
    lexer: SimpleLexicalAnalyzer,
    current_tok: String,
}

impl LolspeakCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
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
            eprintln!("User error: The provided sentence is empty.");
            std::process::exit(1);
        }
    }

    #[inline] fn is_article(&self, s: &str) -> bool { self.lexer.articles.iter().any(|a| a == s) }
    #[inline] fn is_noun(&self, s: &str) -> bool { self.lexer.nouns.iter().any(|n| n == s) }
    #[inline] fn is_verb(&self, s: &str) -> bool { self.lexer.verbs.iter().any(|v| v == s) }
    #[inline] fn is_adverb(&self, s: &str) -> bool { self.lexer.adverbs.iter().any(|adv| adv == s) }
    #[inline] fn is_adjective(&self, s: &str) -> bool { self.lexer.adjectives.iter().any(|adj| adj == s) }

    fn article(&mut self) {
        if self.is_article(&self.current_tok) {
            let _ = self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when an article (a, teh) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn adjective(&mut self) {
        if self.is_adjective(&self.current_tok) {
            let _ = self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when an adjective (fat, hungry, happy, mean) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn noun(&mut self) {
        if self.is_noun(&self.current_tok) {
            let _ = self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when a noun (dawg, kat, rat) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn verb(&mut self) {
        if self.is_verb(&self.current_tok) {
            let _ = self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when a verb (lovez, hatez, ates) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn noun_phrase(&mut self) {
        self.article();
        self.adjective();
        self.noun();
    }

    fn lolspeak(&mut self) {
        self.noun_phrase();
        if self.is_adverb(&self.current_tok) {
            let _ = self.next_token();
        }
        self.verb();
        self.noun_phrase();
    }
}

impl Compiler for LolspeakCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            candidate
        } else if self.lexer.tokens.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    fn parse(&mut self) {
        self.lolspeak();
        if !self.lexer.tokens.is_empty() || !self.current_tok.is_empty() {
            eprintln!("Syntax error: Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    }

    fn current_token(&self) -> String { self.current_tok.clone() }
    fn set_current_token(&mut self, tok: String) { self.current_tok = tok; }
}

//
// ===================== Main =====================
//

fn main() {
    // Usage in VS Code terminal:
    //   cargo run -- lolspeak.txt
    // where lolspeak.txt is in the project root (not src/) of the project.

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let sentence = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolspeakCompiler::new();
    compiler.compile(&sentence);
    compiler.parse();

    println!("The sentence '{}' follows the lolspeak grammar!", sentence.trim());
}
