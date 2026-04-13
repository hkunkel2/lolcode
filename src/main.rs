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

fn main() {
    // Usage in VS Code terminal:
    //   cargo run -- input.lol
    // where input.lol is in the project root (not src/).

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    if !filename.to_ascii_lowercase().ends_with(".lol") {
        eprintln!("Error: input file must have a .lol extension.");
        std::process::exit(1);
    }

    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&source);
    compiler.parse();

    let mut sa = LolcodeSemanticAnalyzer::new();
    let resolved = sa.analyze(&compiler.tree).unwrap_or_else(|| {
        std::process::exit(1);
    });

    let html = HtmlGenerator::new().generate(&resolved);

    let out_path = format!("{}.html", filename.trim_end_matches(".lol").trim_end_matches(".LOL"));
    fs::write(&out_path, &html).unwrap_or_else(|err| {
        eprintln!("Error writing '{}': {}", out_path, err);
        std::process::exit(1);
    });

    open::that(&out_path).unwrap();
}
