use swc_core::ecma::parser::{EsConfig, Syntax};
use testing::fixture;
use use_client::{Config, TransformVisitor};

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
    let config: Config = serde_json::from_str(&config).unwrap();
    let filepath: String = input.to_string_lossy().into();

    test_fixture(
        syntax(),
        &|_tr| {
            as_folder(TransformVisitor {
                filepath: filepath.replace("\\", "/"),
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
