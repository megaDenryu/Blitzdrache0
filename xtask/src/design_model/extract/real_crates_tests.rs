//! リポジトリの実物の `crates` を走査する試験。
//!
//! 他の回帰試験が文字列として組んだソースを使うのは、実物が変わると構文と関係の対応の固定が壊れるためである。
//! ここでは、抽出器が実物のファイル配置と走査の経路で動くことと、実物に在る役割の使用箇所が受理され続けることと、
//! 実物のどの文字列も抽出器の保証範囲の外へ落ちないことを確かめる。
//! 実物で確かめるのは、受理する正規形を狭く書き下した条件が実物の文字列を落とさないことを、文字列の模型では保証できないためである。
//! 保証範囲の外が0件であることは `cargo xtask conform` の `extractable_normal_form` が違反として落とす条件と同じであり、この試験がその偽の違反の不在を先に示す。
//!
//! 走査のルートを`CARGO_MANIFEST_DIR`から組むのは、試験の実行時の作業ディレクトリがxtaskのパッケージであり、`crates`の相対パスが解決しないためである。

use std::path::PathBuf;

use super::outcome::抽出した設計関係グラフと抽出の欠け;
use super::source_group::抽出対象のソース群;
use crate::file_scan;

// 実物の `crates` を走査して抽出の結果を組む。
fn 実物のcratesから結果を組む() -> 抽出した設計関係グラフと抽出の欠け {
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
    super::ソース群から抽出する(&抽出対象のソース群::原文一覧から生成する(原文一覧))
}

#[test]
fn 実物のcratesを走査するとマーカーの実装から下位型である関係が出る() {
    let 結果 = 実物のcratesから結果を組む();
    let 表記一覧: Vec<String> = 結果.グラフ.関係一覧().iter().map(|関係| 関係.表記()).collect();
    assert!(
        表記一覧.contains(&"blitz_esca::traveler::旅行者の現在地 下位型である blitz_design::marker::M状態".to_string()),
        "実物の `impl M状態 for 旅行者の現在地` から関係が出ていない: {}件",
        表記一覧.len()
    );
}

#[test]
fn 実物の3件の役割の使用箇所は受理の条件を満たし処理の節点になる() {
    let 結果 = 実物のcratesから結果を組む();
    let 表記一覧: Vec<String> = 結果.グラフ.概念一覧().iter().map(|概念| 概念.識別子().表記()).collect();
    for 期待 in [
        "blitz_esca::traveler_input::キーボード歩行入力::歩行入力を解釈する",
        "blitz_esca::traveler_movement::旅行者の現在地::歩行を遷移する",
        "blitz_esca::traveler_movement::旅行者の現在地::描画位置を射影する",
    ] {
        assert!(表記一覧.iter().any(|表記| 表記 == 期待), "{期待} が処理の節点として立っていない");
    }
}

#[test]
fn 実物のタプル構造体が包む型へ保持する関係が出る() {
    let 結果 = 実物のcratesから結果を組む();
    let 表記一覧: Vec<String> = 結果.グラフ.関係一覧().iter().map(|関係| 関係.表記()).collect();
    assert!(表記一覧.contains(&"blitz_esca::elapsed_time::経過時間 保持する 秒".to_string()));
    assert!(!結果.グラフ.概念一覧().iter().any(|概念| 概念.識別子().モジュールパス.contains("::tests")));
}

#[test]
fn 実物のcratesに抽出器の保証範囲の外の構文が1件も無い() {
    let 結果 = 実物のcratesから結果を組む();
    let 範囲の外一覧: Vec<String> = 結果.抽出できなかった行一覧.iter().filter(|行| 行.理由.保証範囲の外の構文を採る().is_some()).map(|行| 行.表記()).collect();
    assert!(範囲の外一覧.is_empty(), "保証範囲の外の構文が在る(cargo xtask conform が違反として落とす): {範囲の外一覧:?}");
}

#[test]
fn 実物のcratesに関係を落とした抽出の欠落が1件も無い() {
    let 欠落 = 実物のcratesから結果を組む().関係を落とした抽出の欠落へ写す();
    assert!(!欠落.在るか(), "{}", 欠落.説明());
}

#[test]
fn 実物の遷移関数は遷移パラメータを消費し失敗の型を生成する() {
    let 結果 = 実物のcratesから結果を組む();
    let 表記一覧: Vec<String> = 結果.グラフ.関係一覧().iter().map(|関係| 関係.表記()).collect();
    let 主語 = "blitz_esca::traveler_movement::旅行者の現在地::歩行を遷移する";
    for 期待 in [
        format!("{主語} 消費する blitz_esca::transition_parameter::遷移パラメータ"),
        format!("{主語} 生成する blitz_esca::traveler_error::旅行者の現在地の生成の失敗"),
    ] {
        assert!(表記一覧.contains(&期待), "{期待} が無い");
    }
}

#[test]
fn 実物の関数の役割の型と意味型は宣言したモジュールパスの型の節点として立つ() {
    let 結果 = 実物のcratesから結果を組む();
    let 当たった表記一覧: Vec<String> = 結果.グラフ.概念一覧().iter().map(|概念| 概念.参照を組む()).filter(super::role::関数の役割の型か意味型を指す参照か).map(|参照| 参照.表記()).collect();
    let 期待の件数: usize = super::role::関数の役割::全部の一覧().iter().map(|役割| 1 + 役割.伴う意味型の名前一覧().len()).sum();
    assert_eq!(当たった表記一覧.len(), 期待の件数, "役割の型か意味型の定義の場所が `role` の宣言と食い違っている: {当たった表記一覧:?}");
}
