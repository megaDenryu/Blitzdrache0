//! `blitz_math` の `frame` の原文を読み、座標系を型引数に取る型を名前と座標系の数の組で集める、試験専用の読み取り。
//! 受け取るのは `blitz_math` の `frame` の置き場、返すのはそこに定義された型の名前と座標系の数の組の一覧である。
//!
//! 型引数の名前で座標系を見分けるのは、`blitz_math` の座標系の型引数がどれも `空間` の境界を名乗る名前(空間種・入力空間・出力空間)で書かれているためである。
//! 走査のルートを`CARGO_MANIFEST_DIR`から組むのは、試験の実行時の作業ディレクトリがxtaskのパッケージであり、`crates`の相対パスが解決しないためである。

use std::path::PathBuf;

use crate::conform::design_ontology::line_matching::先頭の識別子;
use crate::file_scan;

/// 座標系を型引数に取る型1つの、名前と型引数に取る座標系の数の組。実物から読んだ側と抽出器の一覧の側が同じ形で並ぶ。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct フレーム型の名前と座標系の数 {
    名前: String,
    座標系の数: usize,
}

impl フレーム型の名前と座標系の数 {
    pub fn 生成する(名前: &str, 座標系の数: usize) -> Self {
        Self { 名前: 名前.to_string(), 座標系の数 }
    }
}

/// `blitz_math` の `frame` のソースが並ぶディレクトリ。
pub struct フレーム型の定義の置き場 {
    ルート: PathBuf,
}

impl フレーム型の定義の置き場 {
    /// このリポジトリの `crates/blitz_math/src/frame` を指す。
    pub fn blitz_mathのframeを指す() -> Self {
        Self {
            ルート: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("crates").join("blitz_math").join("src").join("frame"),
        }
    }

    /// 置き場の全 `.rs` を読み、座標系を型引数に取る型を集める。走査と読み取りの失敗は試験の前提の破れであるため panic で止める。
    pub fn 座標系を型引数に取る型を集める(&self) -> Vec<フレーム型の名前と座標系の数> {
        let Some(ルートの表記) = self.ルート.to_str() else {
            panic!("走査のルートのパスを文字列として読めない: 不変条件「リポジトリのパスはUTF-8である」が破れた");
        };
        let パス一覧 = match file_scan::対象ファイル一覧を集める(&[ルートの表記], &["rs"]) {
            Ok(パス一覧) => パス一覧,
            Err(破れ) => panic!("blitz_mathのframeを走査できなかった: {破れ}"),
        };
        let mut 型一覧 = Vec::new();
        for パス in パス一覧 {
            match std::fs::read_to_string(&パス) {
                Ok(原文) => 型一覧.extend(フレーム型の定義の原文(&原文).座標系を型引数に取る型を集める()),
                Err(誤り) => panic!("blitz_mathのソースを読めなかった: {} {誤り}", パス.display()),
            }
        }
        型一覧
    }
}

// `frame` の1ファイルの原文。
struct フレーム型の定義の原文<'a>(&'a str);

impl フレーム型の定義の原文<'_> {
    // 型引数の名前が全部「空間」を含む公開の構造体と、三成分の量の雛形で定義した型(型引数は座標系1つ)を集める。
    fn 座標系を型引数に取る型を集める(&self) -> Vec<フレーム型の名前と座標系の数> {
        let 行一覧: Vec<&str> = self.0.lines().map(str::trim).collect();
        let mut 型一覧 = Vec::new();
        for (位置, 行) in 行一覧.iter().enumerate() {
            if let Some((名前, 型引数の並び)) = 行.strip_prefix("pub struct ").and_then(|残り| 残り.split_once('<')).and_then(|(名前, 残り)| Some((名前, 残り.split_once('>')?.0))) {
                let 型引数一覧: Vec<String> = 型引数の並び.split(',').map(|型引数| 先頭の識別子(型引数.trim())).collect();
                if !名前.starts_with('$') && 型引数一覧.iter().all(|型引数| 型引数.contains("空間")) {
                    型一覧.push(フレーム型の名前と座標系の数::生成する(名前, 型引数一覧.len()));
                }
            }
            if *行 == "三成分の量を定義する! {"
                && let Some(名前) = 行一覧[位置 + 1..].iter().find(|続き| !続き.starts_with("///")).and_then(|続き| 続き.split_once(',')).map(|(名前, _)| 名前.trim())
            {
                型一覧.push(フレーム型の名前と座標系の数::生成する(名前, 1));
            }
        }
        型一覧
    }
}
