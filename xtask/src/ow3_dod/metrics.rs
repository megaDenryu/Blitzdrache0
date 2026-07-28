//! 統合経路の人間向け要約からDoD判定に必要な整数だけを読み、判定後の1行要約へ整える。

pub(super) struct 計測値 {
    pub(super) 読込件数: u64,
    pub(super) gpu解除件数: u64,
    pub(super) 最大ramバイト数: u64,
    pub(super) 最大vramバイト数: u64,
    pub(super) 最大台帳登録数: u64,
    pub(super) lod変更フレーム数: u64,
    pub(super) lod変更時読込件数: u64,
    pub(super) 最終段種類数: usize,
    pub(super) validation件数: u64,
}

pub(super) fn 読み取る(出力: &str) -> Result<計測値, ()> {
    let 最大使用量行 = 行を探す(出力, "最大使用量:")?;
    let 最大値: Vec<u64> = 最大使用量行.split_whitespace().filter_map(|語| 語.parse().ok()).collect();
    let 最終段行 = 行を探す(出力, "最終フレームの地形詳細段:")?;
    Ok(計測値 {
        読込件数: 先頭整数(行を探す(出力, "ディスク読込:")?)?,
        gpu解除件数: 先頭整数(行を探す(出力, "GPU資源解除:")?)?,
        最大ramバイト数: *最大値.first().ok_or(())?,
        最大vramバイト数: *最大値.get(1).ok_or(())?,
        最大台帳登録数: 先頭整数(行を探す(出力, "最大台帳登録数:")?)?,
        lod変更フレーム数: 先頭整数(行を探す(出力, "LOD変更フレーム数:")?)?,
        lod変更時読込件数: 先頭整数(行を探す(出力, "LOD変更時の読込開始:")?)?,
        最終段種類数: 最終段行.matches("件").count(),
        validation件数: 先頭整数(行を探す(出力, "validationエラー・警告合計件数:")?)?,
    })
}

impl 計測値 {
    pub(super) fn 要約(&self) -> String {
        format!(
            "読込{}件、解除{}件、最大台帳{}件、LOD変更{}フレーム、変更時読込{}件、最終{}段、RAM {} bytes、VRAM {} bytes、validation {}件",
            self.読込件数,
            self.gpu解除件数,
            self.最大台帳登録数,
            self.lod変更フレーム数,
            self.lod変更時読込件数,
            self.最終段種類数,
            self.最大ramバイト数,
            self.最大vramバイト数,
            self.validation件数
        )
    }
}

fn 行を探す<'a>(出力: &'a str, 見出し: &str) -> Result<&'a str, ()> {
    出力.lines().map(str::trim).find(|行| 行.starts_with(見出し)).ok_or(())
}

fn 先頭整数(行: &str) -> Result<u64, ()> {
    行.split_whitespace()
        .find_map(|語| 語.trim_matches(|文字: char| !文字.is_ascii_digit()).parse().ok())
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::先頭整数;

    #[test]
    fn 単位接尾辞つき整数を読む() {
        assert_eq!(先頭整数("ディスク読込: 25件 / 11022050 bytes"), Ok(25));
        assert_eq!(先頭整数("validationエラー・警告合計件数: 0"), Ok(0));
    }
}
