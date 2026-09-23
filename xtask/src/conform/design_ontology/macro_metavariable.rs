//! 表記がマクロのメタ変数(`$名`・`$(..)*`)を含むかを答える純粋な関数。受け取るのは型の表記か名前、返すのは含むかである。
//! `use` と `type` の読み口は、辺を作る名前(`use` の別名とパスの最後の名前、`type` の別名と右辺)にメタ変数があれば読み切れないと答える。展開した先の名前を検査器は知らず、その辺を黙って落とすためである。
//! `$crate` はメタ変数でなく、そのマクロを定義したクレートの根を指す決まったパスであるため数えない(`use $crate::a::規則 as 法則;` は辿れる)。

/// 表記が `$crate` 以外のメタ変数を含むか。
pub fn メタ変数を含むか(表記: &str) -> bool {
    表記.match_indices('$').any(|(位置, _)| {
        let 後ろ = 表記.get(位置 + 1..).unwrap_or_default();
        let crateの後ろ = 後ろ.strip_prefix("crate");
        crateの後ろ.is_none_or(|残り| 残り.chars().next().is_some_and(|文字| 文字.is_alphanumeric() || 文字 == '_'))
    })
}

#[cfg(test)]
mod tests {
    use super::メタ変数を含むか;

    #[test]
    fn crateを除くメタ変数だけを数える() {
        for 表記 in ["$パス", "$元", "crate::a::$名", "$($t)*", "$crate_名", "Vec<$型>"] {
            assert!(メタ変数を含むか(表記), "{表記}");
        }
        for 表記 in ["$crate::a::規則", "crate::a::規則", "Vec<$crate::a::規則>"] {
            assert!(!メタ変数を含むか(表記), "{表記}");
        }
    }
}
