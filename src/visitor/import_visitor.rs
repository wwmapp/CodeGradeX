// src/visitor/import_visitor.rs
use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

pub struct ImportVisitor {
    pub(crate) imports: Vec<String>,
}


impl ImportVisitor {
    pub(crate) fn new() -> Self {
        ImportVisitor { imports: vec![] }
    }
}

impl Visit for ImportVisitor {
    fn visit_import_decl(&mut self, node: &ImportDecl) {
        self.imports.push(node.src.value.to_string());
    }
}