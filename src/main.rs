pub mod lexical_analyzer;
pub mod syntax_analyzer;
pub mod semantic_analyzer;
pub mod html_generator;

use std::env;
use std::fs;
use syntax_analyzer::{LolcodeCompiler};
use semantic_analyzer::{LolcodeSemanticAnalyzer, SemanticAnalyzer};
use html_generator::HtmlGenerator;

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
// ===================== Main =====================
//
/// lolcode compiler 
/// 
/// This is a compiler for this grammer 
/// 
/// ==============================================================================================================
/// 
/// Terminals
/// 
/// HAI ::= "#HAI"
/// 
/// KBYE ::= "#KBYE"
/// 
/// OBTW ::= "#OBTW"
/// 
/// TLDR ::= "#TLDR"
/// 
/// MAEK ::= "#MAEK"
/// 
/// OIC ::= "#OIC"
/// 
/// GIMMEH ::= "#GIMMEH"
/// 
/// MKAY ::= "#MKAY"
/// 
/// HEAD ::= "HEAD"
/// 
/// TITLE ::= "TITLE"
/// 
/// PARAGRAF ::= "PARAGRAF"
/// 
/// BOLD ::= "BOLD"
/// 
/// ITALICS ::= "ITALICS"
/// 
/// LIST ::= "LIST"
/// 
/// ITEM ::= "ITEM"
/// 
/// NEWLINE ::= "NEWLINE"
/// 
/// LINX ::= "LINX"
/// 
/// IHAZ ::= "#IHAZ"
/// 
/// ITIZ ::= "#ITIZ"
/// 
/// LEMMESEE ::= "#LEMMESEE"
/// 
/// VARNAME ::= any single word (A–Z, a–z, no spaces)
/// 
/// VARVALUE ::= allowed text characters
/// 
/// TEXT ::= plain text (letters, digits, punctuation, spaces)
/// 
/// ADDRESS ::= text without spaces
/// 
/// * Note that these terminals are not case sensitive.
/// * * The only allowed plain text in our language is: A-Z, a-z, 0-9, commas, period, quotes, colons, question marks, exclamation points, percent signs,
/// forward slashes, equals sign, and hidden special characters for tabs and newlines.
/// Syntax/Production Rules (i.e., non-terminals)
/// 
/// <lolcode> ::= HAI <comments> < head> <body> KBYE
/// 
/// <comments> ::= <comment> <comments>
/// 
/// | ε
/// 
/// <comment> ::= OBTW TEXT TLDR
/// 
/// <head> ::= MAEK HEAD <title> MKAY
/// 
/// | ε
/// 
/// <title> ::= GIMMEH TITLE TEXT OIC
/// 
/// <body> ::= <inner-body> <body>
/// 
/// | ε
/// 
/// <inner-body> ::= <paragraph>
/// 
/// | <comment>
/// 
/// | <bold>
/// 
/// | <italics>
/// 
/// | <list>
/// 
/// | <link>
/// 
/// | <newline>
/// 
/// | <variable-define>
/// 
/// | <variable-use>
/// 
/// | TEXT
/// 
/// <paragraph> ::= MAEK PARAGRAF <variable-define> <inner-paragraph> MKAY
/// 
/// <inner-paragraph> ::= <inner-text> <inner-paragraph>
/// 
/// | ε
/// 
/// <inner-text> ::= <variable-use>
/// 
/// | <bold>
/// 
/// | <italics>
/// 
/// | <list>
/// 
/// | <link>
/// 
/// | <newline>
/// 
/// | TEXT
/// 
/// | ε
/// 
/// <bold> ::= GIMMEH BOLD TEXT OIC
/// 
/// <italics> ::= GIMMEH ITALICS TEXT OIC
/// 
/// <list> ::= MAEK LIST <list-items> MKAY
/// 
/// <list-items> ::= <list-item> <list-items>
/// 
/// | ε
/// 
/// <list-item> ::= GIMMEH ITEM <inner-list> OIC
/// 
/// <inner-list> ::= <bold>
/// 
/// | <italics>
/// 
/// | <link>
/// 
/// | TEXT
/// 
/// | <variable-use>
/// 
/// | ε
/// 
/// <link> ::= GIMMEH LINX ADDRESS OIC
/// 
/// <newline> ::= GIMMEH NEWLINE
/// 
/// <variable-define>::= IHAZ VARNAME ITIZ VARVALUE MKAY
/// 
/// | ε
/// 
/// <variable-use> ::= LEMMESEE VARNAME OIC
/// 
/// ==============================================================================================================
/// 
/// Usage: 
/// 
/// cargo run -- <file-path>

fn main() {
    // Usage in VS Code terminal:
    //   cargo run -- input.lol
    // where input.lol is in the project root (not src/).


    // gets args
    let args: Vec<String> = env::args().collect();
    // checks that args is less than 2 and errors if more 
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // sets arg to var filename
    let filename = &args[1];

    // validates .lol extension
    if !filename.to_ascii_lowercase().ends_with(".lol") {
        eprintln!("Error: input file must have a .lol extension.");
        std::process::exit(1);
    }

    // gets markdown from file, if fails errors
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    // new lolcode compiler instance as compiler 
    let mut compiler = LolcodeCompiler::new();
    // gives instance file data in constructor
    compiler.compile(&source);
    // parses data 
    compiler.parse();

    // instancate sematic analyzer 
    let mut sa = LolcodeSemanticAnalyzer::new();
    // sematic analyzer checks for static errors and returns syntax tree but with resolved variable references with values
    let resolved = sa.analyze(&compiler.tree).unwrap_or_else(|| {
        std::process::exit(1);
    });
    // generates html data 
    let html = HtmlGenerator::new().generate(&resolved);
    // creates html file path 
    let out_path = format!("{}.html", filename.trim_end_matches(".lol").trim_end_matches(".LOL"));
    // creates html file 
    fs::write(&out_path, &html).unwrap_or_else(|err| {
        eprintln!("Error writing '{}': {}", out_path, err);
        std::process::exit(1);
    });

    // opens html
    open::that(&out_path).unwrap();
}
