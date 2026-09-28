//! `モジュールパス` の試験。ファイルの配置からの導出と、親の導出を確かめる。

use std::path::Path;

use super::module_path::モジュールパス;

fn 組む(区切り一覧: &[&str]) -> モジュールパス {
    モジュールパス::区切り一覧から組む(区切り一覧)
}

#[test]
fn モジュールパス_libとmodは区切りにならず入れ子は二重コロンで繋ぐ() {
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("crates/a/src/lib.rs")), 組む(&["a"]));
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("crates/a/src/main.rs")), 組む(&["a"]));
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("crates/a/src/x/y.rs")), 組む(&["a", "x", "y"]));
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("crates/a/src/x/mod.rs")), 組む(&["a", "x"]));
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("C:/devs/repo/crates/a/src/tests/x_tests.rs")), 組む(&["a", "tests", "x_tests"]));
    assert_eq!(モジュールパス::ファイルのパスから求める(Path::new("xtask/src/x.rs")), 組む(&[]));
}

#[test]
fn モジュールパス_最上位の親は自分自身() {
    assert_eq!(組む(&["a", "x", "y"]).親(), 組む(&["a", "x"]));
    assert_eq!(組む(&["a"]).親(), 組む(&["a"]));
}

#[test]
fn モジュールパス_クレートと下へ繋ぐ() {
    assert_eq!(組む(&["a", "x", "y"]).クレート(), 組む(&["a"]));
    assert_eq!(組む(&["a"]).下へ繋ぐ(&["x", "y"]), 組む(&["a", "x", "y"]));
    assert_eq!(組む(&["a"]).下へ繋ぐ(&[]), 組む(&["a"]));
}
