use swc_common::{
    FileName,
    SourceMap,
};
use swc_common::sync::Lrc;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig};

fn main() {
    // 创建 SourceMap，这是解析时所需的
    let cm: Lrc<SourceMap> = Default::default();

    // 示例 JavaScript 代码
    let source_code = "let x = 1;";

    // 创建输入源
    let fm = cm.new_source_file(
        FileName::Custom("input.js".to_string()),
        source_code.to_string(),
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
    match parser.parse_script() {
        Ok(script) => {
            println!("{:#?}", script);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}