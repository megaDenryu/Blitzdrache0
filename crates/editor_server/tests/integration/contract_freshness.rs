//! TypeScriptの型契約が最新であることの検査。生成物をリポジトリへコミットする方式は、
//! 同じ内容を2箇所へ置く二重台帳を作る。ここで文字列一致を強制し、写しを手で直す
//! 抜け道も同時に塞ぐ。参照: GameScriptingTheoryの`editor_server/tests/contract_freshness.rs`
//! (同じ形の仕掛け)。このテストはファイルを書き換えない。書き換える実装は乖離を隠すため受け入れない。
#![cfg(feature = "typescript")]
#![allow(clippy::unwrap_used)]

use std::path::Path;

#[test]
fn 生存確認契約tsは組み立てた本文と一致する() {
    最新であることを確かめる("生存確認契約.ts", editor_server::契約ファイルの本文を組み立てる());
}

#[test]
fn 編集資源契約tsは組み立てた本文と一致する() {
    最新であることを確かめる("編集資源契約.ts", editor_server::編集資源契約の本文を組み立てる());
}

fn 最新であることを確かめる(ファイル名: &str, 組み立てた本文: String) {
    let 出力先 = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../editor_web/src/生成").join(ファイル名);
    let ディスクの内容 = std::fs::read_to_string(&出力先).unwrap();
    assert_eq!(
        ディスクの内容, 組み立てた本文,
        "editor_web/src/生成/{ファイル名} が最新でない。リポジトリルートで \
         `cargo xtask contract-export` を実行して再生成する"
    );
}
