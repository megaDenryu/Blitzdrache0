//! アプリの`--report-streaming-summary`が出す人間向け要約から、検収が使う整数だけを読む工程。
//! 受け取るのは標準出力、返すのは転送量・使用量・台帳登録数・LOD変更の数である。
//! 行の綴りは`crates/blitz_app/src/reports/streaming_summary.rs`の出力と一致させている。
//! ストリーミングを通す検収が2つ以上あるため、同じ行を2箇所で読まないようここへ集める。

pub struct ストリーミング要約報告 {
    pub ディスク読込件数: u64,
    pub ディスク読込バイト数: u64,
    pub gpu解除件数: u64,
    pub 最大ramバイト数: u64,
    pub 最大vramバイト数: u64,
    pub 最大台帳登録数: u64,
    /// 最終フレームの台帳登録数。最大が実行の途中の値であるのに対し、これは終状態を表すため、
    /// 解除を経た実行に不要な束が残っているかどうかを一度も解除していない実行と突き合わせられる。
    pub 最終台帳登録数: u64,
    pub lod変更フレーム数: u64,
    pub lod変更時読込開始件数: u64,
    /// 最終フレームに立っていた地形詳細段の種類数。段の分布の行に現れた「件」の数で数える。
    pub 最終段種類数: usize,
}

pub fn 取り出す(標準出力: &str) -> Result<ストリーミング要約報告, String> {
    let 読込行 = 行を探す(標準出力, "ディスク読込:")?;
    let 最大使用量行 = 行を探す(標準出力, "最大使用量:")?;
    let 数値一覧: Vec<u64> = 最大使用量行.split_whitespace().filter_map(|語| 語.parse().ok()).collect();
    Ok(ストリーミング要約報告 {
        ディスク読込件数: 先頭整数(読込行)?,
        ディスク読込バイト数: 末尾整数(読込行)?,
        gpu解除件数: 先頭整数(行を探す(標準出力, "GPU資源解除:")?)?,
        最大ramバイト数: *数値一覧.first().ok_or_else(|| "最大使用量の行にRAMの数が無い".to_string())?,
        最大vramバイト数: *数値一覧.get(1).ok_or_else(|| "最大使用量の行にVRAMの数が無い".to_string())?,
        最大台帳登録数: 先頭整数(行を探す(標準出力, "最大台帳登録数:")?)?,
        最終台帳登録数: 先頭整数(行を探す(標準出力, "最終台帳登録数:")?)?,
        lod変更フレーム数: 先頭整数(行を探す(標準出力, "LOD変更フレーム数:")?)?,
        lod変更時読込開始件数: 先頭整数(行を探す(標準出力, "LOD変更時の読込開始:")?)?,
        最終段種類数: 行を探す(標準出力, "最終フレームの地形詳細段:")?.matches("件").count(),
    })
}

fn 行を探す<'a>(標準出力: &'a str, 見出し: &str) -> Result<&'a str, String> {
    標準出力
        .lines()
        .map(str::trim)
        .find(|行| 行.starts_with(見出し))
        .ok_or_else(|| format!("ストリーミング要約に「{見出し}」の行が無い"))
}

/// 「25件 / 11155300 bytes」のような行の最初の整数。単位の接尾辞を落としてから数として読む。
pub fn 先頭整数(行: &str) -> Result<u64, String> {
    整数を並べる(行).into_iter().next().ok_or_else(|| format!("「{行}」に整数が無い"))
}

fn 末尾整数(行: &str) -> Result<u64, String> {
    整数を並べる(行).into_iter().next_back().ok_or_else(|| format!("「{行}」に整数が無い"))
}

fn 整数を並べる(行: &str) -> Vec<u64> {
    行.split_whitespace()
        .filter_map(|語| 語.trim_matches(|文字: char| !文字.is_ascii_digit()).parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{先頭整数, 末尾整数};

    #[test]
    fn 単位接尾辞つき整数を読む() {
        assert_eq!(先頭整数("ディスク読込: 25件 / 11155300 bytes"), Ok(25));
        assert_eq!(末尾整数("ディスク読込: 25件 / 11155300 bytes"), Ok(11_155_300));
        assert_eq!(先頭整数("GPU資源解除: 0件"), Ok(0));
    }
}
