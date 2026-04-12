pub mod lexical_analyzer;
pub mod syntax_analyzer;
pub mod semantic_analyzer;

use std::env;
use std::fs;
use syntax_analyzer::{LolcodeCompiler, Node};
use semantic_analyzer::{LolcodeSemanticAnalyzer, SemanticAnalyzer};

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

fn print_tree(nodes: &[Node], depth: usize) {
    let indent = "  ".repeat(depth);
    for node in nodes {
        match node {
            Node::Str(s)      => println!("{}{}", indent, s),
            Node::List(items) => {
                println!("{}[", indent);
                print_tree(items, depth + 1);
                println!("{}]", indent);
            }
        }
    }
}

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
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&source);
    compiler.parse();

    println!("Parse successful! '{}' follows the lolcode grammar.", filename);
    print_tree(&compiler.tree, 0);

    let mut sa = LolcodeSemanticAnalyzer::new();
    let resolved = sa.analyze(&compiler.tree).unwrap_or_else(|| {
        std::process::exit(1);
    });

    println!("Semantic analysis passed. Resolved tree:");
    print_tree(&resolved, 0);
}
