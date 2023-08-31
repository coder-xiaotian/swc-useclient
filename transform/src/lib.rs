use serde::Deserialize;
use swc_core::{
    common::{BytePos, Span, SyntaxContext},
    ecma::{
        ast::{ExprStmt, Module, ModuleItem, Stmt, Str},
        visit::{VisitMut, VisitMutWith},
    },
};

#[derive(Deserialize, Debug)]
pub struct IncludeConfig {
    path: String,
    #[serde(default = "default_insert")]
    insert: String,
}
fn default_insert() -> String {
    String::from("use client")
}
pub struct TransformVisitor {
    pub filepath: String,
    pub include: Vec<IncludeConfig>,
}
impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, n: &mut Module) {
        for include_config in &self.include {
            if self.filepath.contains(include_config.path.as_str()) {
                let mut raw = include_config.insert.clone();
                raw.insert_str(0, "\"");
                raw.insert_str(raw.len(), "\"");
                let str = Str {
                    span: Span {
                        lo: BytePos(0),
                        hi: BytePos(12),
                        ctxt: SyntaxContext::from_u32(0),
                    },
                    value: include_config.insert.clone().into(),
                    raw: Option::Some(raw.into()),
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
        n.visit_mut_children_with(self);
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Include {
    Str(String),
    Obj(IncludeConfig),
}
#[derive(Deserialize, Debug)]
pub struct UserConfig {
    include: Vec<Include>
}
pub fn normalize_config(config: &UserConfig) -> Vec<IncludeConfig> {
    let mut configs = vec![];
    for include in &config.include {
        configs.push(match include {
            Include::Obj(o) => IncludeConfig {path: o.path.to_string(), insert: o.insert.to_string()},
            Include::Str(s) => IncludeConfig {path: s.to_string(), insert: "use client".into()}
        })
    }
    configs
}