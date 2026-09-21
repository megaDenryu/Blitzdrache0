//! `struct` と `enum` の宣言の行と、その本体の中のフィールドの記述を読む工程。依存を持たない純粋な関数だけを置く。
//! 受け取るのはコードだけの1行、返すのは型の名前と型引数の名前の一覧、またはフィールドの記述の一覧である。
//!
//! この読み取りを規則3の本体から分けるのは、抽象度の層が違うためである。規則3は「どの型のどのフィールドから何の関係を出すか」を書き、こちらは Rust の構文を読む。
//! 型引数の名前を宣言から読むのは、フィールドの型がその名前と一致するときに関係を出さないためである。出すと実在しない設計概念の節点ができ、
//! 同じモジュールの別の宣言の型引数が同じ表記へ潰れて1つの節点へ合流し、グラフが意味を失う。
//!
//! 保証範囲: 宣言が1行に収まっていること、フィールドが1行に1件であること、1行に収めた本体(`歩く { 方向: 歩行方向 }`)の中の型が型引数の区切りの読点を含まないことを前提にする。

use crate::conform::design_ontology::line_matching::先頭の識別子;

/// `struct` または `enum` の宣言1件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 型定義の宣言 {
    pub 名前: String,
    pub 型引数名一覧: Vec<String>, // `<状態: M状態, イベント: Mイベント>` なら `状態` と `イベント`
}

/// その行が `struct` または `enum` の宣言なら、名前と型引数の名前を読む。
pub fn 型定義の宣言を読む(行: &str) -> Option<型定義の宣言> {
    let 前置きを落とした = 可視性を落とす(行.trim());
    let 残り = ["struct ", "enum "].iter().find_map(|種別| 前置きを落とした.strip_prefix(種別))?.trim_start();
    let 名前 = 先頭の識別子(残り);
    if 名前.is_empty() {
        return None;
    }
    Some(型定義の宣言 {
        名前,
        型引数名一覧: 型引数名一覧を読む(&残り[先頭の識別子(残り).len()..]),
    })
}

/// その行に在るフィールドの記述(`名前: 型` の形)の一覧。波括弧を含む行は括弧の中だけを読み、含まない行は行そのものを1件として読む。
/// 宣言の行は波括弧より前を捨てるため、`struct X<T>(T);` のような名前を持たない本体からは1件も読まない。
pub fn フィールドの記述一覧(行: &str, 宣言の行か: bool) -> Vec<String> {
    if let Some((_, 波括弧の後ろ)) = 行.split_once('{') {
        let 中身 = 波括弧の後ろ.rsplit_once('}').map_or(波括弧の後ろ, |(中身, _)| 中身);
        return 中身.split(',').map(str::trim).filter(|記述| !記述.is_empty()).map(str::to_string).collect();
    }
    if 宣言の行か {
        return Vec::new();
    }
    let 記述 = 行.trim().trim_end_matches(',').trim();
    if 記述.is_empty() { Vec::new() } else { vec![記述.to_string()] }
}

/// フィールドの記述から名前と型の表記を読む。`名前: 型` の形でなければ無しである。
pub fn フィールドの名前と型(記述: &str) -> Option<(String, String)> {
    let (名前側, 型側) = 可視性を落とす(記述.trim()).split_once(':')?;
    let 名前 = 名前側.trim().to_string();
    if 名前.is_empty() || 先頭の識別子(&名前) != 名前 {
        return None;
    }
    Some((名前, 型側.trim().trim_end_matches(',').trim().to_string()))
}

// `pub`・`pub(crate)`・`pub(super)` の前置きを落とす。
fn 可視性を落とす(記述: &str) -> &str {
    let mut 残り = 記述;
    for 前置き in ["pub(crate) ", "pub(super) ", "pub "] {
        残り = 残り.strip_prefix(前置き).unwrap_or(残り).trim_start();
    }
    残り
}

// `<状態: M状態, イベント: Mイベント>` から型引数の名前だけを読む。型引数が無ければ空である。
fn 型引数名一覧を読む(名前の後ろ: &str) -> Vec<String> {
    let Some(型引数) = 名前の後ろ.trim_start().strip_prefix('<') else {
        return Vec::new();
    };
    let 中身 = 型引数.split_once('>').map_or(型引数, |(中身, _)| 中身);
    中身.split(',').map(|項| 項.split_once(':').map_or(項, |(名前, _)| 名前).trim()).filter(|名前| !名前.is_empty()).map(str::to_string).collect()
}
