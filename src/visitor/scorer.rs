// src/visitor/scorer.rs
use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

pub struct Scorer {
    score: i32,
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
