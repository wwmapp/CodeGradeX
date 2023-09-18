use swc_common::{
    FileName,
    SourceMap,
};
use swc_common::sync::Lrc;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use swc_ecma_ast::*;
use swc_ecma_visit::Visit;
struct Scorer {
    score: i32,
}
struct ImportVisitor {
    imports: Vec<String>,
}
impl Scorer {
    fn new() -> Self {
        Scorer { score: 0 }
    }
}

impl Visit for Scorer {
    fn visit_fn_decl(&mut self, f: &FnDecl) {
        let base_score = 10;
        let mut func_score = base_score;

        // 减分: 如果函数超过10行
        let line_span = f.function.span;
        let line_diff = line_span.hi.0 - line_span.lo.0;
        if line_diff > 10 {
            func_score -= 1;
        }

        // 减分: 如果函数使用了 `any` 类型
        for param in &f.function.params {
            if let Pat::Ident(pat_ident) = &param.pat {
                if let Some(ref ts_type_ann) = pat_ident.type_ann {
                    if let TsType::TsKeywordType(keyword) = &*ts_type_ann.type_ann {
                        if let TsKeywordTypeKind::TsAnyKeyword = keyword.kind {
                            func_score -= 2;
                        }
                    }
                }
            }
        }

        self.score += func_score;
    }
}
impl ImportVisitor {
    fn new() -> Self {
        ImportVisitor { imports: vec![] }
    }
}

impl Visit for ImportVisitor {
    fn visit_import_decl(&mut self, node: &ImportDecl) {
        self.imports.push(node.src.value.to_string());
    }
}

fn main() {
    print!("Please enter the path to the input file: ");
    io::stdout().flush().unwrap(); // 使得上面的print!立即显示，不需要等待换行

    let mut path_to_file = String::new();
    io::stdin().read_line(&mut path_to_file).expect("Failed to read line");
    let path_to_file = path_to_file.trim(); // 移除任何的尾随换行或空格

    let source_code = match fs::read_to_string(Path::new(&path_to_file)) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {:?}", err);
            return;
        }
    };

    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Custom(path_to_file.to_string()),
        source_code,
    );

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

    // 解析源代码
    match parser.parse_module() {
        Ok(module) => {
            println!("{:#?}", module);
            let mut visitor = ImportVisitor::new();
            visitor.visit_module(&module);

            for import in &visitor.imports {
                println!("Imported: {}", import);
            }
            let mut scorer = Scorer::new();
            scorer.visit_module(&module);

            println!("Score: {}", scorer.score);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

}