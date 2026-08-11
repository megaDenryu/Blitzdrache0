//! 生成器とコンパイラが報告する行から、焼き直したチャンクの本数を読む工程。受け取るのは取り込んだ標準出力、
//! 返すのは焼き直した本数である。
//!
//! 注意: 読み取る綴りは書き手(`crates/blitz_asset_compiler/src/generation_ledger/rebake_tally.rs`)にも同じものがある。
//! 食い違えば「報告の行が無い」で失敗する。黙って0本と読まないのは、増分が効いていないことを効いていると
//! 取り違えるほうが害が大きいためである。

const 本数の欄の綴り: &str = "焼き直したチャンク数=";

pub(super) fn 焼き直した本数を読む(標準出力: &str, 誰の報告か: &str) -> Result<usize, String> {
    let 見つけた行 = 標準出力
        .lines()
        .find_map(|行| 行.split(本数の欄の綴り).nth(1))
        .ok_or_else(|| format!("{誰の報告か}の出力に{本数の欄の綴り}の行が無い"))?;
    let 本数の綴り = 見つけた行.split_whitespace().next().unwrap_or_default();
    本数の綴り
        .parse::<usize>()
        .map_err(|誤り| format!("{誰の報告か}の焼き直した本数を数として読めない({本数の綴り}): {誤り}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 報告の行から本数を読む() {
        let 出力 = "[compile_assets] 焼いた\n[compile_assets] 焼き直したチャンク数=3 据え置いたチャンク数=6\n";
        assert_eq!(焼き直した本数を読む(出力, "コンパイラ").unwrap(), 3);
    }

    #[test]
    fn 報告の行が無ければ失敗する() {
        assert!(焼き直した本数を読む("[compile_assets] 焼いた\n", "コンパイラ").is_err());
    }
}
