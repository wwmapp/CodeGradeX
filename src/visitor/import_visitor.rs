use std::fs;
use std::path::Path;
use swc_common::input::StringInput;
use swc_common::{FileName, SourceMap};
use swc_common::sync::Lrc;
// src/visitor/import_visitor.rs
use swc_ecma_ast::*;
use swc_ecma_parser::{Parser, Syntax, TsConfig};
use swc_ecma_visit::Visit;

pub struct ImportVisitor {
    pub imports: Vec<String>,
    pub visited_files: Vec<String>,
}


impl ImportVisitor {

    pub fn new() -> Self {
        ImportVisitor { imports: vec![], visited_files: vec![] }
    }

    pub(crate) fn process_file(&mut self, cm: &Lrc<SourceMap>, path: &str) -> Result<(), String> {
        if self.visited_files.contains(&path.to_string()) {
            return Ok(());
        }

        self.visited_files.push(path.to_string());

        // 打印当前正在处理的文件路径
        println!("Processing file: {}", path);

        let source_code = fs::read_to_string(Path::new(path)).map_err(|e| format!("{:?}", e))?;

        let fm = cm.new_source_file(FileName::Custom(path.to_string()), source_code);

        // 创建解析器
        let lexer = swc_ecma_parser::lexer::Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                dts: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: false,
            }),
            swc_ecma_ast::EsVersion::EsNext,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        let module = parser.parse_module().map_err(|e| format!("{:?}", e))?;
        self.visit_module(&module);

        let current_imports = self.imports.clone();
        for import in &current_imports {
            if import.starts_with(".") {
                println!("Processing import: {}", import);
                self.process_file(cm, import)?;
            }else {
                println!("Skipping absolute import: {}", import);
            }
        }

        Ok(())
    }
    fn try_resolve_module(&self, path: &str) -> Option<String> {
        let possible_extensions = ["", ".js", ".jsx", ".ts", ".tsx", ".json", ".vue"];

        for ext in &possible_extensions {
            let full_path = format!("{}{}", path, ext);
            println!("Trying path: {}", full_path);
            if Path::new(&full_path).exists() {
                return Some(full_path);
            }
        }

        None
    }
}

impl Visit for ImportVisitor {
    fn visit_import_decl(&mut self, node: &ImportDecl) {
        if let Some(resolved_path) = self.try_resolve_module(&node.src.value.to_string()) {
            self.imports.push(resolved_path);
        } else {
            // 可选: 处理不能解析的模块导入，例如通过日志记录或其他方式
            eprintln!("Failed to resolve module: {}", node.src.value);
        }
    }
}

// src/visitor/scorer.rs