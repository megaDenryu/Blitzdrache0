//! `blitz_app`が出す「空代表画素」の行を読む工程。受け取るのは標準出力の全文、返すのは行ごとの構造である。
//! 書式は`crates/blitz_app/src/reports/sky_pixel/row.rs`とこの工程の間の契約であり、`atmosphere_lut/parse`と
//! 同じ「鍵=値」の語を空白で切る書式に揃えてある。

pub(super) struct 代表画素行 {
    pub(super) 横: u32,
    pub(super) 縦: u32,
    pub(super) 期待: [f64; 3],
    pub(super) 実測: [f64; 3],
}

pub(super) fn 読み取る(標準出力: &str) -> Result<Vec<代表画素行>, String> {
    標準出力.lines().filter(|行| 行.starts_with("空代表画素 ")).map(一行を読む).collect()
}

fn 一行を読む(行: &str) -> Result<代表画素行, String> {
    let 語一覧: Vec<&str> = 行.split_whitespace().collect();
    Ok(代表画素行 {
        横: 値を読む(&語一覧, "横")?
            .parse()
            .map_err(|誤り| format!("横を整数として読めない: {誤り}"))?,
        縦: 値を読む(&語一覧, "縦")?
            .parse()
            .map_err(|誤り| format!("縦を整数として読めない: {誤り}"))?,
        期待: [
            実数を読む(&語一覧, "期待赤")?,
            実数を読む(&語一覧, "期待緑")?,
            実数を読む(&語一覧, "期待青")?,
        ],
        実測: [
            実数を読む(&語一覧, "実測赤")?,
            実数を読む(&語一覧, "実測緑")?,
            実数を読む(&語一覧, "実測青")?,
        ],
    })
}

fn 値を読む<'a>(語一覧: &[&'a str], 鍵: &str) -> Result<&'a str, String> {
    let 接頭辞 = format!("{鍵}=");
    語一覧
        .iter()
        .find_map(|語| 語.strip_prefix(接頭辞.as_str()))
        .ok_or_else(|| format!("空代表画素の行に{鍵}が無い"))
}

fn 実数を読む(語一覧: &[&str], 鍵: &str) -> Result<f64, String> {
    値を読む(語一覧, 鍵)?.parse().map_err(|誤り| format!("{鍵}を実数として読めない: {誤り}"))
}
