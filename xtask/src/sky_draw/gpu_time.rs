//! パス別GPU時間の報告から空パスの平均ミリ秒を読む工程。
//! 受け取るのはblitz_appの標準出力、返すのは空パスの移動平均ミリ秒である。
//! 行の形は「  空: 0.0154 ms」であり、パス名はレンダラーのパス宣言と一致する。

const 見出し: &str = "パス別GPU時間";
const パス名: &str = "空:";

pub(super) fn 空パスの平均msを読む(標準出力: &str) -> Result<f64, String> {
    let mut 行一覧 = 標準出力.lines().skip_while(|行| !行.contains(見出し));
    行一覧.next().ok_or_else(|| format!("標準出力に{見出し}の区画が無い"))?;
    let 行 = 行一覧
        .take_while(|行| 行.starts_with("  "))
        .find(|行| 行.trim_start().starts_with(パス名))
        .ok_or_else(|| format!("{見出し}の区画に{パス名}の行が無い"))?;
    let 値 = 行
        .trim_start()
        .trim_start_matches(パス名)
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("{パス名}の行に値が無い: {行}"))?;
    値.parse().map_err(|誤り| format!("{パス名}の値を数として読めない({行}): {誤り}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const 標本: &str = "パス別GPU時間(移動平均、60フレーム窓):\n  シーン描画: 0.0105 ms\n  空: 0.0154 ms\n  シャドウ: 0.0050 ms\n";

    #[test]
    fn 空パスの行から平均msを読む() {
        assert!((空パスの平均msを読む(標本).unwrap() - 0.0154).abs() < 1e-9);
    }

    #[test]
    fn 区画が無ければ失敗にする() {
        assert!(空パスの平均msを読む("何もない出力\n").is_err());
    }

    #[test]
    fn 区画に空の行が無ければ失敗にする() {
        assert!(空パスの平均msを読む("パス別GPU時間:\n  シーン描画: 0.0105 ms\n").is_err());
    }
}
