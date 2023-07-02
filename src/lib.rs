use swc_core::{ecma::{
    visit::{Fold, FoldWith},
}, plugin::metadata::TransformPluginMetadataContextKind};
use swc_core::ecma::ast::{
    Module,
    ExprStmt,
    Str,
    ModuleItem,
    Program,
    Stmt
};
use swc_core::common::{
    Span,
    BytePos,
    SyntaxContext,
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor {
    filepath: String
}

impl Fold for TransformVisitor {
    fn fold_module(&mut self, mut n: Module) -> Module {
        if self.filepath == "@mui" {
            let str = Str {
                span: Span {
                    lo: BytePos(0),
                    hi: BytePos(12),
                    ctxt: SyntaxContext::from_u32(0)
                },
                value: "use client".into(),
                raw: Option::Some("\"use client\"".into())
            };
            let e = ExprStmt {
                span: Span {
                    lo: BytePos(0),
                    hi: BytePos(12),
                    ctxt: SyntaxContext::from_u32(0)
                },
                expr: Box::new(str.into())
            };
            n.body.insert(0, ModuleItem::Stmt(Stmt::Expr(e)));
        }

        n
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from("")
    };
    
    program.fold_with(&mut TransformVisitor { filepath: filepath })
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
// test!(
//     Default::default(),
//     |_| as_folder(TransformVisitor),
//     boo,
//     // Input codes
//     r#"
//     function test() {}
//     "#,
//     r#"
//     use client
//     function test() {}
//     "#
// );
