use std::{fs::read_to_string, path::PathBuf};
use swc_core::{
    ecma::transforms::testing::{test_fixture, FixtureTestConfig},
    ecma::{
        parser::{EsConfig, Syntax},
        visit::as_folder,
    },
};
use testing::fixture;
use use_client::{UserConfig, TransformVisitor, normalize_config};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixtures/**/input.js")]
fn use_client_fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let output = dir.join("output.js");
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    let config: UserConfig = serde_json::from_str(&config).unwrap();
    let filepath: String = input.to_string_lossy().into();

    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor {
                filepath: filepath.replace("\\", "/"),
                include: normalize_config(&config),
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
