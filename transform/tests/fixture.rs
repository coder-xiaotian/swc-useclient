use std::path::PathBuf;

use use_client::TransformVisitor;
use swc_core::{
    ecma::{
        visit::as_folder,
        parser::{EsConfig, Syntax}},
    ecma::transforms::testing::{test_fixture, FixtureTestConfig},
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/input.js")]
fn use_client_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
          as_folder(TransformVisitor {
            filepath: "@mui".into()
          })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
