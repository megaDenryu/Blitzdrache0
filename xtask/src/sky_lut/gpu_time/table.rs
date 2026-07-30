//! パス別GPU時間の報告の表を読む工程。受け取るのは標準出力、返すのは区間名とミリ秒の対の一覧である。
//!
//! 表の書式をここが単独で持つのは、これが`--report-gpu-times`の出力との唯一の接点だからである。
//! 見出しの後に続く2桁の字下げの行が表であり、字下げが切れたところで表が終わる。

const 見出し: &str = "パス別GPU時間";

pub(super) fn 読む(標準出力: &str) -> Result<Vec<(&str, f64)>, String> {
    let mut 行一覧 = 標準出力.lines().skip_while(|行| !行.contains(見出し));
    行一覧.next().ok_or_else(|| format!("標準出力に{見出し}の区画が無い"))?;
    let mut 表 = Vec::new();
    for 行 in 行一覧.take_while(|行| 行.starts_with("  ")) {
        let 中身 = 行.trim_start();
        let Some((名前, 残り)) = 中身.split_once(':') else {
            continue;
        };
        let 値 = 残り.split_whitespace().next().unwrap_or_default();
        let ミリ秒 = 値.parse().map_err(|誤り| format!("{名前}の値を数として読めない({行}): {誤り}"))?;
        表.push((名前, ミリ秒));
    }
    Ok(表)
}

/// 名前で区間を引く。欠けているのは、そのパスが積まれていないか区間名が変わったことを意味するため失敗にする。
pub(super) fn 区間を引く(表: &[(&str, f64)], 区間名: &str) -> Result<f64, String> {
    表.iter()
        .find(|(名前, _)| *名前 == 区間名)
        .map(|(_, ミリ秒)| *ミリ秒)
        .ok_or_else(|| format!("{見出し}の表に区間{区間名}が無い"))
}
