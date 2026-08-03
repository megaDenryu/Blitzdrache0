//! 遠方環境報告の全文から、判定に要る行だけを取り出す工程。受け取るのは標準出力、返すのは行の値を組んだ報告である。
//!
//! 行の形をここが単独で持つのは、これがblitz_appとxtaskの間の機械可読な契約そのものだからである。
//! 報告側へ項目を足すときに直す場所が1つになる。語の並びに依存せず鍵で参照するのは、報告へ項目を足したときに読み取り側が壊れないようにするためである。行の値の取り出しは`field`が持つ。

mod field;

use field::{実数値, 整数値, 添字値, 語句値};

/// 鍵の判定の1行。条件の名前とそのときの指示である。
pub(super) struct 鍵行 {
    pub(super) 条件: String,
    pub(super) 指示: String,
}

/// 代表テクセルの1行。期待と実測の3成分を持つ。
pub(super) struct 代表行 {
    pub(super) 層: u32,
    pub(super) 横: u32,
    pub(super) 縦: u32,
    pub(super) 期待: [f64; 3],
    pub(super) 実測: [f64; 3],
}

pub(super) struct 報告 {
    pub(super) 検証の状況: String,
    pub(super) 検証件数: usize,
    pub(super) 円盤対照の不一致: usize,
    pub(super) テクセル数: usize,
    pub(super) 非有限: usize,
    pub(super) 負: usize,
    pub(super) 予約が非零: usize,
    pub(super) 面境界の最大相対差: f64,
    pub(super) 鍵一覧: Vec<鍵行>,
    pub(super) 代表一覧: Vec<代表行>,
}

pub(super) fn 報告を取り出す(標準出力: &str) -> Result<報告, String> {
    let mut 鍵一覧 = Vec::new();
    let mut 代表一覧 = Vec::new();
    let mut 検証 = None;
    let mut 円盤対照 = None;
    let mut 統計 = None;
    let mut 面境界 = None;
    for 行 in 標準出力.lines() {
        let 語一覧: Vec<&str> = 行.split_whitespace().collect();
        match 語一覧.first().copied() {
            Some("遠方環境鍵") => 鍵一覧.push(鍵行 {
                条件: 語句値(&語一覧, "条件")?,
                指示: 語句値(&語一覧, "指示")?,
            }),
            Some("検証層") => 検証 = Some((語句値(&語一覧, "状況")?, 整数値(&語一覧, "件数")?)),
            Some("遠方環境円盤対照") => 円盤対照 = Some(整数値(&語一覧, "不一致")?),
            Some("遠方環境統計") => {
                統計 = Some((
                    整数値(&語一覧, "テクセル数")?,
                    整数値(&語一覧, "非有限")?,
                    整数値(&語一覧, "負")?,
                    整数値(&語一覧, "予約が非零")?,
                ));
            }
            Some("遠方環境面境界") => 面境界 = Some(実数値(&語一覧, "最大相対差")?),
            Some("遠方環境代表") => 代表一覧.push(代表行を読む(&語一覧)?),
            _ => {}
        }
    }
    let (検証の状況, 検証件数) = 検証.ok_or_else(|| "報告に検証層の行が無い".to_string())?;
    let (テクセル数, 非有限, 負, 予約が非零) = 統計.ok_or_else(|| "報告に遠方環境統計の行が無い".to_string())?;
    Ok(報告 {
        検証の状況,
        検証件数,
        円盤対照の不一致: 円盤対照.ok_or_else(|| "報告に遠方環境円盤対照の行が無い".to_string())?,
        テクセル数,
        非有限,
        負,
        予約が非零,
        面境界の最大相対差: 面境界.ok_or_else(|| "報告に遠方環境面境界の行が無い".to_string())?,
        鍵一覧,
        代表一覧,
    })
}

fn 代表行を読む(語一覧: &[&str]) -> Result<代表行, String> {
    Ok(代表行 {
        層: 添字値(語一覧, "層")?,
        横: 添字値(語一覧, "横")?,
        縦: 添字値(語一覧, "縦")?,
        期待: [実数値(語一覧, "期待赤")?, 実数値(語一覧, "期待緑")?, 実数値(語一覧, "期待青")?],
        実測: [実数値(語一覧, "実測赤")?, 実数値(語一覧, "実測緑")?, 実数値(語一覧, "実測青")?],
    })
}
