//! blitz_appの終了時報告からvalidation件数を読む工程。受け取るのは標準出力と条件の名前、返すのは0件であったかどうかである。
//! 読み戻し画像を使う検収はどれも同じ行を同じ判定で読むため、数え方と失敗の文面を1箇所に置く。

/// blitz_appが終了時に出す件数の行の見出し。読む側の綴りをここが1つ持ち、他の入口はこれを参照する。
/// 綴りはblitz_appの出力と一致していなければならず、食い違いは`cargo xtask conform`の綴りの契約の検査が拒む。
pub const 検証層の指摘件数の見出し: &str = "validationエラー・警告合計件数:";

pub fn 零件数を確かめる(標準出力: &str, 条件名: &str) -> Result<(), String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.contains(検証層の指摘件数の見出し))
        .ok_or_else(|| format!("{条件名}の出力にvalidation件数の行が無い"))?;
    let 件数: u64 = 行
        .rsplit(':')
        .next()
        .unwrap_or_default()
        .trim()
        .parse()
        .map_err(|誤り| format!("{条件名}のvalidation件数を数として読めない: {誤り}"))?;
    if 件数 != 0 {
        return Err(format!("{条件名}でvalidationが{件数}件発生した"));
    }
    Ok(())
}
