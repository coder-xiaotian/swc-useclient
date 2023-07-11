use serde::Deserialize;
use swc_core::{
    common::{BytePos, Span, SyntaxContext},
    ecma::{
        ast::{ExprStmt, Module, ModuleItem, Stmt, Str},
        visit::VisitMut,
    },
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub include: Vec<String>,
}
pub struct TransformVisitor {
    pub filepath: String,
    pub include: Vec<String>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, n: &mut Module) {
        for path in &self.include {
            if self.filepath.contains(path) {
                let str = Str {
                    span: Span {
                        lo: BytePos(0),
                        hi: BytePos(12),
                        ctxt: SyntaxContext::from_u32(0),
                    },
                    value: "use client".into(),
                    raw: Option::Some("\"use client\"".into()),
                };
                let e = ExprStmt {
                    span: Span {
                        lo: BytePos(0),
                        hi: BytePos(12),
                        ctxt: SyntaxContext::from_u32(0),
                    },
                    expr: Box::new(str.into()),
                };
                n.body.insert(0, ModuleItem::Stmt(Stmt::Expr(e)));
            }
        }
    }
}
