mod visitor;
use swc_common::{
    FileName,
    SourceMap,
};
use swc_common::sync::Lrc;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use swc_ecma_visit::Visit;
use visitor::{scorer::Scorer, import_visitor::ImportVisitor};
fn to_absolute_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    current_dir.join(relative_path)
}
fn main() {
    print!("Please enter the path to the input file: ");
    io::stdout().flush().unwrap();

    let mut path_to_file = String::new();
    io::stdin().read_line(&mut path_to_file).expect("Failed to read line");
    let path_to_file = path_to_file.trim();
    let abs_path = to_absolute_path(path_to_file);
    println!("Absolute path: {}", abs_path.display());
    let cm: Lrc<SourceMap> = Default::default();

    let mut visitor = ImportVisitor::new();
    if let Err(error) = visitor.process_file(&cm, &path_to_file) {
        eprintln!("Error processing file: {}", error);
        return;
    }

    for import in &visitor.imports {
        println!("Imported: {}", import);
    }

    let mut scorer = Scorer::new();
    for file in &visitor.visited_files {
        let source_code = fs::read_to_string(file).expect("Failed to read file");
        let fm = cm.new_source_file(FileName::Custom(file.to_string()), source_code);
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
        let module = parser.parse_module().unwrap();
        scorer.visit_module(&module);
    }

    println!("Score: {}", scorer.score);
}
