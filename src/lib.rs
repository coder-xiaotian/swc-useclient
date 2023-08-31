use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        metadata::{TransformPluginMetadataContextKind, TransformPluginProgramMetadata},
        plugin_transform,
    },
};

use use_client::{UserConfig, TransformVisitor, normalize_config};

#[plugin_transform]
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<UserConfig>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for use-client"),
    )
    .expect("invalid config for use-client");
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s.replace("\\", "/"),
        None => String::from(""),
    };
    let include = normalize_config(&config);

    program.visit_mut_with(&mut TransformVisitor {
        filepath: filepath,
        include,
    });

    program
}