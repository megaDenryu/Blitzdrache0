//! リポジトリの実物の `crates` を走査する試験。1件だけ置く。
//!
//! 他の回帰試験が文字列として組んだソースを使うのは、実物が変わると構文と関係の対応の固定が壊れるためである。
//! この1件だけは、抽出器が実物のファイル配置と走査の経路で動くことを確かめる。判定は、実物に必ず在る関係が1件出ることに限る。
//!
//! 走査のルートを`CARGO_MANIFEST_DIR`から組むのは、試験の実行時の作業ディレクトリがxtaskのパッケージであり、`crates`の相対パスが解決しないためである。

use std::path::PathBuf;

use super::source_group::抽出対象のソース群;
use crate::file_scan;

#[test]
fn 実物のcratesを走査するとマーカーの実装から下位型である関係が出る() {
    let ルート = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("crates");
    let Some(ルートの表記) = ルート.to_str().map(str::to_string) else {
        panic!("走査のルートのパスを文字列として読めない: 不変条件「リポジトリのパスはUTF-8である」が破れた");
    };
    let パス一覧 = match file_scan::対象ファイル一覧を集める(&[&ルートの表記], &["rs"]) {
        Ok(パス一覧) => パス一覧,
        Err(破れ) => panic!("実物のcratesを走査できなかった: {破れ}"),
    };
    let mut 原文一覧 = Vec::new();
    for パス in パス一覧.into_iter().filter(|パス| パス.components().any(|部品| 部品.as_os_str() == "src")) {
        match std::fs::read_to_string(&パス) {
            Ok(原文) => 原文一覧.push((パス, 原文)),
            Err(誤り) => panic!("実物のソースを読めなかった: {} {誤り}", パス.display()),
        }
    }
    assert!(!原文一覧.is_empty(), "実物のcratesのソースが1件も集まらなかった: {ルートの表記}");
    let 結末 = super::ソース群から抽出する(&抽出対象のソース群::原文一覧から生成する(原文一覧));
    let 表記一覧: Vec<String> = 結末.グラフ.関係一覧().iter().map(|関係| 関係.表記()).collect();
    assert!(
        表記一覧.contains(&"blitz_esca::traveler::旅行者の現在地 下位型である blitz_design::marker::M状態".to_string()),
        "実物の `impl M状態 for 旅行者の現在地` から関係が出ていない: {}件",
        表記一覧.len()
    );
}
