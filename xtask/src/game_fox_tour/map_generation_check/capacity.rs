//! 遠景の頂点・インデックス容量が世界の広がりから一意に決まることを確かめる。

use blitz_asset_compiler::世界の広がり;

pub(super) fn 遠景容量を確かめる(報告: &str, 広がり: 世界の広がり) -> Result<String, String> {
    let 東セル数 = u64::from(広がり.東西チャンク数()) * 4;
    let 南セル数 = u64::from(広がり.南北チャンク数()) * 4;
    let 期待頂点数 = (東セル数 + 1)
        .checked_mul(南セル数 + 1)
        .ok_or_else(|| "期待頂点数が桁あふれした".to_string())?;
    let 期待インデックス数 = 東セル数
        .checked_mul(南セル数)
        .and_then(|値| 値.checked_mul(6))
        .ok_or_else(|| "期待インデックス数が桁あふれした".to_string())?;
    let 頂点数 = 数を読む(報告, "頂点数=")?;
    let インデックス数 = 数を読む(報告, "インデックス数=")?;
    if 頂点数 != 期待頂点数 {
        return Err(format!("頂点数は{期待頂点数}のはずだが{頂点数}"));
    }
    if インデックス数 != 期待インデックス数 {
        return Err(format!("インデックス数は{期待インデックス数}のはずだが{インデックス数}"));
    }
    Ok(format!("遠景容量は頂点{頂点数}・インデックス{インデックス数}で広がりと一致した"))
}

fn 数を読む(報告: &str, 見出し: &str) -> Result<u64, String> {
    let 後ろ = 報告
        .split_once(見出し)
        .map(|(_, 後ろ)| 後ろ)
        .ok_or_else(|| format!("報告に{見出し}が無い"))?;
    let 綴り = 後ろ.split_whitespace().next().ok_or_else(|| format!("{見出し}の値が無い"))?;
    綴り.parse::<u64>().map_err(|誤り| format!("{見出し}{綴り}を数にできない: {誤り}"))
}
