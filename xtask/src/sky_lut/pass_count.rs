//! 大気LUT生成パス数の報告行を読む工程。受け取るのは標準出力と条件の名前、返すのはフレームごとの本数の列である。
//! 行の綴りは`crates/blitz_app/src/reports/atmosphere_passes.rs`の出力と一致させている。

const 見出し: &str = "大気LUT生成パス数";

pub(super) struct 生成パス数の列 {
    pub(super) 数えたフレーム数: u64,
    pub(super) 総数: u64,
    pub(super) フレームごと: Vec<u32>,
}

impl 生成パス数の列 {
    /// 先頭から数フレームぶんの並び。報告の文面へ載せて、判定が見た形をそのまま示す。
    pub(super) fn 先頭の並び(&self, 件数: usize) -> Vec<u32> {
        self.フレームごと.iter().copied().take(件数).collect()
    }

    /// 指定の本数だったフレームの回数。
    pub(super) fn 本数の回数(&self, 本数: u32) -> usize {
        self.フレームごと.iter().filter(|値| **値 == 本数).count()
    }
}

pub(super) fn 読む(標準出力: &str, 条件名: &str) -> Result<生成パス数の列, String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.starts_with(見出し))
        .ok_or_else(|| format!("{条件名}の出力に{見出し}の行が無い"))?;
    Ok(生成パス数の列 {
        数えたフレーム数: 数を読む(行, "数えたフレーム数=", 条件名)?,
        総数: 数を読む(行, "総数=", 条件名)?,
        フレームごと: 列を読む(行, 条件名)?,
    })
}

fn 数を読む(行: &str, 前置き: &str, 条件名: &str) -> Result<u64, String> {
    let 残り = 行
        .split(前置き)
        .nth(1)
        .ok_or_else(|| format!("{条件名}の{見出し}の行に{前置き}が無い: {行}"))?;
    let 語 = 残り.split_whitespace().next().unwrap_or_default();
    語.parse().map_err(|誤り| format!("{条件名}の{前置き}を数として読めない({語}): {誤り}"))
}

fn 列を読む(行: &str, 条件名: &str) -> Result<Vec<u32>, String> {
    let 残り = 行
        .split("列=")
        .nth(1)
        .ok_or_else(|| format!("{条件名}の{見出し}の行に列が無い: {行}"))?
        .trim();
    if 残り.is_empty() {
        return Err(format!("{条件名}の{見出し}の列が空である"));
    }
    残り
        .split(',')
        .map(|語| {
            語.parse::<u32>()
                .map_err(|誤り| format!("{条件名}の列の要素を数として読めない({語}): {誤り}"))
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const 標本: &str = "大気LUT生成パス数 数えたフレーム数=4 総数=3 列の長さ=4 列=3,0,0,0\n";

    #[test]
    fn 報告行から列を読む() {
        let 列 = 読む(標本, "標本").unwrap();
        assert_eq!(列.数えたフレーム数, 4);
        assert_eq!(列.総数, 3);
        assert_eq!(列.フレームごと, vec![3, 0, 0, 0]);
        assert_eq!(列.本数の回数(0), 3);
    }

    #[test]
    fn 行が無ければ失敗にする() {
        assert!(読む("何もない出力\n", "標本").is_err());
    }
}
