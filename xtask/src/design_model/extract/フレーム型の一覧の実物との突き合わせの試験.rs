//! フレーム型の一覧が `blitz_math` の実物の座標系を型引数に取る型と過不足なく一致することを、実物のソースを走査して確かめる試験。
//!
//! 一覧は抽出器が `blitz_math` の外から書き写した名前であり、`blitz_math` へフレーム型を足しても一覧は自動では増えない。
//! 足し忘れると、足した型の表記が保証範囲の外へ落ちて、その型を保持する関係が欠落する。この試験はその食い違いを先に落とす。
//! 走査のルートを`CARGO_MANIFEST_DIR`から組むのは、試験の実行時の作業ディレクトリがxtaskのパッケージであり、`crates`の相対パスが解決しないためである。

use std::path::PathBuf;

use super::表記が名指す型::フレーム型;
use crate::conform::design_ontology::line_matching::先頭の識別子;
use crate::file_scan;

#[test]
fn フレーム型の一覧はblitz_mathの座標系を型引数に取る型と過不足なく一致する() {
    let ルート = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("crates").join("blitz_math").join("src").join("frame");
    let Some(ルートの表記) = ルート.to_str().map(str::to_string) else {
        panic!("走査のルートのパスを文字列として読めない: 不変条件「リポジトリのパスはUTF-8である」が破れた");
    };
    let パス一覧 = match file_scan::対象ファイル一覧を集める(&[&ルートの表記], &["rs"]) {
        Ok(パス一覧) => パス一覧,
        Err(破れ) => panic!("blitz_mathのframeを走査できなかった: {破れ}"),
    };
    let mut 実物の型一覧 = Vec::new();
    for パス in パス一覧 {
        match std::fs::read_to_string(&パス) {
            Ok(原文) => 実物の型一覧.extend(座標系を型引数に取る型を集める(&原文)),
            Err(誤り) => panic!("blitz_mathのソースを読めなかった: {} {誤り}", パス.display()),
        }
    }
    let mut 一覧の型一覧: Vec<String> = フレーム型::一覧().map(|型| format!("{}<座標系{}個>", 型.名前(), 型.座標系の数())).collect();
    実物の型一覧.sort();
    一覧の型一覧.sort();
    assert_eq!(一覧の型一覧, 実物の型一覧, "フレーム型の一覧が blitz_math の実物と一致しない");
}

// 原文から、型引数の名前が全部「空間」を含む公開の構造体と、三成分の量の雛形で定義した型(型引数は座標系1つ)を `名前<座標系N個>` の形で集める。
// 型引数の名前で座標系を見分けるのは、`blitz_math` の座標系の型引数がどれも `空間` の境界を名乗る名前(空間種・入力空間・出力空間)で書かれているためである。
fn 座標系を型引数に取る型を集める(原文: &str) -> Vec<String> {
    let 行一覧: Vec<&str> = 原文.lines().map(str::trim).collect();
    let mut 型一覧 = Vec::new();
    for (位置, 行) in 行一覧.iter().enumerate() {
        if let Some((名前, 型引数の並び)) = 行.strip_prefix("pub struct ").and_then(|残り| 残り.split_once('<')).and_then(|(名前, 残り)| Some((名前, 残り.split_once('>')?.0))) {
            let 型引数一覧: Vec<String> = 型引数の並び.split(',').map(|型引数| 先頭の識別子(型引数.trim())).collect();
            if !名前.starts_with('$') && 型引数一覧.iter().all(|型引数| 型引数.contains("空間")) {
                型一覧.push(format!("{名前}<座標系{}個>", 型引数一覧.len()));
            }
        }
        if *行 == "三成分の量を定義する! {" {
            if let Some(名前) = 行一覧[位置 + 1..].iter().find(|続き| !続き.starts_with("///")).and_then(|続き| 続き.split_once(',')).map(|(名前, _)| 名前.trim()) {
                型一覧.push(format!("{名前}<座標系1個>"));
            }
        }
    }
    型一覧
}
