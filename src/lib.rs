use std::{os, path::PathBuf};

use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        metadata::{TransformPluginMetadataContextKind, TransformPluginProgramMetadata},
        plugin_transform,
    },
};
use use_client::{Config, TransformVisitor};

#[plugin_transform]
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for styled-components"),
    )
    .expect("invalid config for use-client");
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s.replace("\\", "/"),
        None => String::from(""),
    };
    println!("---- filepath -----\n{}", filepath);

    program.visit_mut_with(&mut TransformVisitor {
        filepath: filepath,
        include: config.include,
    });

    program
}
