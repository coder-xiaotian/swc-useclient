use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};
use swc_core::{
    ecma::transforms::testing::{test_fixture, FixtureTestConfig},
    ecma::{
        parser::{EsConfig, Syntax},
        visit::as_folder,
    },
};
use testing::fixture;
use use_client::TransformVisitor;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestConfig {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub filepath: String,
}

#[fixture("tests/fixtures/**/input.js")]
fn use_client_fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let output = dir.join("output.js");
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Config -----\n{}", config);
    let config: TestConfig = serde_json::from_str(&config).unwrap();

    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor {
                filepath: config.filepath.clone(),
                include: config.include.to_vec(),
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
