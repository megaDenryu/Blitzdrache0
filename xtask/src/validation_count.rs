//! blitz_appの終了時報告からvalidation件数を読む工程。受け取るのは標準出力と読み手の名前、
//! 返すのは件数か、0件であったかどうかである。
//! 読み戻し画像を使う検収はどれも同じ行を同じ判定で読むため、数え方と失敗の文面を1箇所に置く。

use crate::report_heading::報告の見出し;

/// blitz_appが終了時に出す件数の行の見出し。読む側の綴りをここが1つ持ち、他の入口はこれを参照する。
/// 綴りはblitz_appの出力と一致していなければならず、食い違いは`cargo xtask conform`の綴りの契約の検査が拒む。
pub const 検証層の指摘件数の見出し: 報告の見出し = 報告の見出し::生成する("validationエラー・警告合計件数:");

pub fn 件数を読む(標準出力: &str, 読み手の名前: &str) -> Result<u64, String> {
    let 行 = 検証層の指摘件数の見出し
        .行を探す(標準出力)
        .ok_or_else(|| format!("{読み手の名前}の出力にvalidation件数の行が無い"))?;
    検証層の指摘件数の見出し
        .見出しに続く本文(行)
        .parse()
        .map_err(|誤り| format!("{読み手の名前}のvalidation件数を数として読めない: {誤り}"))
}

pub fn 零件数を確かめる(標準出力: &str, 条件名: &str) -> Result<(), String> {
    let 件数 = 件数を読む(標準出力, 条件名)?;
    if 件数 != 0 {
        return Err(format!("{条件名}でvalidationが{件数}件発生した"));
    }
    Ok(())
}
