use std::collections::HashMap;
use crate::syntax_analyzer::Node;

// ===================== SemanticAnalyzer Trait =====================

pub trait SemanticAnalyzer {
    /// Walk the parse tree, resolve all variable uses to their values, and
    /// return the transformed tree.  Returns None if a semantic error is found.
    fn analyze(&mut self, tree: &[Node]) -> Option<Vec<Node>>;
}

// ===================== Struct =====================

pub struct LolcodeSemanticAnalyzer {
    /// Stack of scopes — innermost scope is last.
    /// Each scope maps variable name → value.
    scopes: Vec<HashMap<String, String>>,
}

// ===================== Impl =====================

impl LolcodeSemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // global scope
        }
    }

    /// Walk a slice of nodes, returning the resolved version.
    /// #ihaz nodes are consumed (they define variables, produce no output).
    /// #lemmesee nodes are replaced by the variable's value string.
    /// All other nodes are rebuilt recursively.
    fn resolve_nodes(&mut self, nodes: &[Node]) -> Option<Vec<Node>> {
        let mut out = Vec::new();
        for node in nodes {
            match node {
                // pushes if just string since var definitions cant live on root
                Node::Str(s) => out.push(Node::Str(s.clone())),
                Node::List(items) => {
                    match self.resolve_list(items)? {
                        Some(resolved) => out.push(resolved),
                        None => {} // #ihaz — consumed, produces no output node
                    }
                }
            }
        }
        Some(out)
    }

    /// Resolve one list node.
    /// Returns:
    ///   None            — semantic error (propagates up)
    ///   Some(None)      — node was consumed (#ihaz), nothing added to parent
    ///   Some(Some(node))— replacement node to add to parent
    fn resolve_list(&mut self, items: &[Node]) -> Option<Option<Node>> {
        // Identify the construct from the first two string tokens.
        // iter of nodes and filter out non-strings grabs frist 2 because we only care about 
        // creating a block for scope, setting a var value or resolving a var references
        let mut strs = items.iter().filter_map(|n| {
            if let Node::Str(s) = n { Some(s.to_ascii_lowercase()) } else { None }
        });
        let first  = strs.next().unwrap_or_default();
        let second = strs.next().unwrap_or_default();

        match (first.as_str(), second.as_str()) {
            // --- variable definition: register, produce no output node ---
            // 
            ("#ihaz", _) => {
                // passes current block with items to handle_define which will set the var value for the scope
                self.handle_define(items)?;
                Some(None) // consumed
            }

            // --- variable use: replace with the stored value ---
            ("#lemmesee", _) => {
                // // passes current block with items to handle_use which will set the var value for the var reference
                let value = self.handle_use(items)?;
                Some(Some(Node::Str(value)))
            }

            // --- paragraf / list blocks: new scope, recurse, rebuild ---
            ("#maek", "paragraf") | ("#maek", "list") => {
                self.scopes.push(HashMap::new()); //creates a new block
                let resolved_items = self.resolve_nodes(items)?; // recurively calls resolve node for these tags since they create new scope blocks 
                self.scopes.pop(); // block ends remove local scope 
                Some(Some(Node::List(resolved_items))) // return resolved items
            }

            // --- everything else: recurse in same scope, rebuild ---
            _ => {
                let resolved_items = self.resolve_nodes(items)?;// move through current scope
                Some(Some(Node::List(resolved_items))) // returns resolved items
            }
        }
    }

    /// Register a variable in the current (innermost) scope.
    /// List shape: [Str("#ihaz"), Str(name), Str("#itiz"), Str(value), Str("#mkay")]
    fn handle_define(&mut self, items: &[Node]) -> Option<()> {
        let name = match items.get(1) {
            Some(Node::Str(s)) => s.clone(),
            _ => return Some(()),
        };
        let value = match items.get(3) {
            Some(Node::Str(s)) => s.clone(),
            _ => String::new(),
        };
        self.scopes.last_mut().unwrap().insert(name, value);
        Some(())
    }

    /// Look up a variable through the scope stack (innermost first).
    /// Returns the value on success, None (with error printed) on failure.
    /// List shape: [Str("#lemmesee"), Str(name), Str("#oic")]
    fn handle_use(&mut self, items: &[Node]) -> Option<String> {
        // gets var name
        let name = match items.get(1) {
            Some(Node::Str(s)) => s.clone(),
            _ => return Some(String::new()),
        };
        // sets var value based on scope walks up to global from current scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(&name) {
                return Some(value.clone());
            }
        }

        eprintln!(
            "Semantic error: undefined variable '{}'",
            name,
        );
        None
    }
}

// ===================== Trait Impl =====================

impl SemanticAnalyzer for LolcodeSemanticAnalyzer {
    fn analyze(&mut self, tree: &[Node]) -> Option<Vec<Node>> {
        self.resolve_nodes(tree)
    }
}
