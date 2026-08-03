//! blitz_appが出した板ごとの照合行を数の並びへ直す工程。受け取るのは標準出力、返すのは板の行の一覧である。
//! 判定はこの並びだけを見る。
//!
//! 行が1本も無いことを失敗にするのは、注入も報告も行われなかった実行を「全項目が通った」と読み替えないためである。

/// 板1枚ぶんの照合結果。
pub(super) struct 板の行 {
    pub(super) 番号: usize,
    pub(super) 期待: [f64; 3],
    pub(super) 実測: [f64; 3],
    pub(super) 面: u32,
    pub(super) 段の位置: f64,
    pub(super) 面の端からの余裕: f64,
}

const 行の見出し: &str = "間接照明代表板 ";

pub(super) fn 板の行を取り出す(標準出力: &str) -> Result<Vec<板の行>, String> {
    let mut 一覧 = Vec::new();
    for 行 in 標準出力.lines().filter(|行| 行.trim_start().starts_with(行の見出し)) {
        if 行.contains("照合できなかった") {
            return Err(format!("照合できなかった板がある: {}", 行.trim()));
        }
        一覧.push(板の1行を読む(行.trim())?);
    }
    if 一覧.is_empty() {
        return Err("板の照合行が1本も出ていない".to_string());
    }
    Ok(一覧)
}

fn 板の1行を読む(行: &str) -> Result<板の行, String> {
    Ok(板の行 {
        番号: usize::try_from(数を読む(行, "番号=")?).map_err(|誤り| format!("板の番号がusizeに収まらない: {誤り}"))?,
        期待: 色の3成分を読む(行, "期待=")?,
        実測: 色の3成分を読む(行, "実測=")?,
        面: u32::try_from(数を読む(行, "面=")?).map_err(|誤り| format!("面の番号がu32に収まらない: {誤り}"))?,
        段の位置: 実数を読む(行, "段=")?,
        面の端からの余裕: 実数を読む(行, "面余裕=")?,
    })
}

fn 語を取り出す<'a>(行: &'a str, 語の見出し: &str) -> Result<&'a str, String> {
    行.split_whitespace()
        .find_map(|語| 語.strip_prefix(語の見出し))
        .ok_or_else(|| format!("行に「{語の見出し}」が無い: {行}"))
}

fn 実数を読む(行: &str, 語の見出し: &str) -> Result<f64, String> {
    let 語 = 語を取り出す(行, 語の見出し)?;
    語.parse::<f64>()
        .map_err(|誤り| format!("「{語の見出し}」の値を数として読めない({語}): {誤り}"))
}

fn 数を読む(行: &str, 語の見出し: &str) -> Result<u64, String> {
    let 語 = 語を取り出す(行, 語の見出し)?;
    語.parse::<u64>()
        .map_err(|誤り| format!("「{語の見出し}」の値を整数として読めない({語}): {誤り}"))
}

fn 色の3成分を読む(行: &str, 語の見出し: &str) -> Result<[f64; 3], String> {
    let 語 = 語を取り出す(行, 語の見出し)?;
    let 成分一覧: Vec<f64> = 語
        .split(',')
        .map(|値| {
            値.parse::<f64>()
                .map_err(|誤り| format!("「{語の見出し}」の成分を数として読めない({値}): {誤り}"))
        })
        .collect::<Result<Vec<f64>, String>>()?;
    if 成分一覧.len() != 3 {
        return Err(format!("「{語の見出し}」の成分が3つでない: {語}"));
    }
    Ok([成分一覧[0], 成分一覧[1], 成分一覧[2]])
}
