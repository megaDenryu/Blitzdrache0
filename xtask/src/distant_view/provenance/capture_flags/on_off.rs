//! 旗1つぶんの2状態の綴りと、その読み書き。受け取るのは真偽または旗の行の本文、返すのは綴りまたは読み取った真偽である。
//!
//! 綴りを`on`と`off`に固定し、読めない綴りを黙って偽へ倒さない。倒すと、綴りを書き誤った採取が
//! 「切ってある」という意味に化けて判定を通り抜ける。
//!
//! 欄そのものが無い場合だけは記録なしとして返す。欄を足す前に採った由来がそれであり、その欄を実際に要る
//! 段だけが失敗にできるようにする。欄はあるのに綴りが不正な場合は記録なしと同じには扱わない。

pub(super) fn 入切(使うか: bool) -> &'static str {
    if 使うか { "on" } else { "off" }
}

/// 欄が無ければ記録が無いものとして返す。欄はあるのに綴りが不正なら、記録が無いのと同じには扱わず失敗にする。
pub(super) fn 記録があれば入切を読む(本文: &str, 前置き: &str) -> Result<Option<bool>, String> {
    if !本文.split_whitespace().any(|語| 語.starts_with(前置き)) {
        return Ok(None);
    }
    入切を読む(本文, 前置き).map(Some)
}

pub(super) fn 入切を読む(本文: &str, 前置き: &str) -> Result<bool, String> {
    let 綴り = 本文
        .split_whitespace()
        .find_map(|語| 語.strip_prefix(前置き))
        .ok_or_else(|| format!("採取の旗に{前置き}が無い: {本文}"))?;
    match 綴り {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(format!("採取の旗の{前置き}がonでもoffでもない: {綴り}")),
    }
}
