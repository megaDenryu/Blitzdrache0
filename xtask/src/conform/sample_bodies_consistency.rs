//! 材質見本の立体の宣言が、正本と検収側の写しで同じ値であることの突き合わせ。受け取るのは無し(対象の宣言は
//! この表が持つ)、返すのは値が食い違った宣言の違反一覧である。
//!
//! シェーダー定数の写し検査と別に置くのは、こちらが1行1値でなく配列の宣言を丸ごと比べるためである。
//! 球の粗さの水準は8組の並びであり、1行1値の読み手では組の並びと個数の食い違いを捉えられない。
//!
//! 片方だけを直した食い違いは走らせても落ちない。検収は写しの粗さを報告の名前に使うだけであるため、
//! 正本が粗さを変えても検収は前の水準の名前で同じ球を報告し、誤った表が正しい形で出続ける。

use std::path::{Path, PathBuf};

use super::violation::違反;

const 正本パス: &str = "crates/blitz_asset_compiler/examples/generate_source_assets/terrain_visual_world/sample_bodies.rs";
const 写しパス: &str = "xtask/src/sample_world_region/sample_bodies.rs";

/// 突き合わせる宣言の名前。正本と写しで同じ名前を使うため、前置きは1つで足りる。
const 宣言名一覧: [&str; 3] = ["const 金属の色", "const 誘電体の色", "const 材質の水準一覧"];

pub fn 全宣言を検査する() -> Result<Vec<違反>, String> {
    let 正本 = std::fs::read_to_string(Path::new(正本パス)).map_err(|誤り| format!("{正本パス}の読み取りに失敗した: {誤り}"))?;
    let 写し = std::fs::read_to_string(Path::new(写しパス)).map_err(|誤り| format!("{写しパス}の読み取りに失敗した: {誤り}"))?;
    let mut 違反一覧 = Vec::new();
    for 宣言名 in 宣言名一覧 {
        let 正本の値 = 値を読む(&正本, 宣言名, 正本パス)?;
        let 写しの値 = 値を読む(&写し, 宣言名, 写しパス)?;
        if 正本の値 != 写しの値 {
            違反一覧.push(違反::ファイル単位(
                PathBuf::from(写しパス),
                format!("{宣言名}の写しが正本と食い違う(正本{正本の値:?}・写し{写しの値:?}・正本は{正本パス})"),
            ));
        }
    }
    Ok(違反一覧)
}

/// 宣言の等号から終端の`];`までに現れる数をすべて読む。等号より後だけを見るのは、型の`[f32; 3]`や
/// `[(f32, f32); 球の個数]`に含まれる数を値と取り違えないためである。
/// 宣言が見つからない場合を違反でなく失敗として扱うのは、宣言の消失を「一致した」と読み替えないためである。
fn 値を読む(内容: &str, 宣言名: &str, パス: &str) -> Result<Vec<f64>, String> {
    let 開始 = 内容.find(宣言名).ok_or_else(|| format!("{パス}に「{宣言名}」の宣言が無い"))?;
    let 残り = &内容[開始..];
    let 等号の後 = 残り.find('=').ok_or_else(|| format!("{パス}の「{宣言名}」に等号が無い"))?;
    let 終端 = 残り[等号の後..]
        .find("];")
        .ok_or_else(|| format!("{パス}の「{宣言名}」が`];`で終わっていない"))?;
    数を集める(&残り[等号の後..等号の後 + 終端], 宣言名, パス)
}

fn 数を集める(本文: &str, 宣言名: &str, パス: &str) -> Result<Vec<f64>, String> {
    let mut 一覧 = Vec::new();
    for 語 in 本文.split(|文字: char| !文字.is_ascii_digit() && 文字 != '.' && 文字 != '-') {
        if 語.is_empty() {
            continue;
        }
        一覧.push(
            語.parse::<f64>()
                .map_err(|誤り| format!("{パス}の「{宣言名}」の値を数として読めない: {語}({誤り})"))?,
        );
    }
    if 一覧.is_empty() {
        return Err(format!("{パス}の「{宣言名}」に値が1つも無い"));
    }
    Ok(一覧)
}
