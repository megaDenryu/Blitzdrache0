//! 表記の中に名前が識別子の境界で現れるかを答える純粋な関数。受け取るのは表記と名前、返すのは現れるかである。
//! 識別子の境界とは、その現れの直前の文字と直後の文字がどちらも識別子の文字(英数字・下線・非ASCIIの文字)でない位置のことである。識別子の文字の判定 `識別子の文字か` は、設計オントロジーの検査器の全体がこの1つを使う。
//! 自己変更の禁止の検査は、実装の対象の表記にマーカーの名前がこの形で現れるかで実装を集める。単純な部分文字列の一致にしないのは、日本語の識別子に語の区切りが無く、`歩数を動かす領域` が `歩数` で当たるためである。
//! 直前を空白と `)` に限る判定(`line_matching.rs` の `語として現れるか`)も流用しない。あの判定では `impl 変更 for crate::歩行::地点` の `地点` が `::` の後ろにあるため当たらない。

/// 表記の中に名前が識別子の境界で現れるか。名前が空なら偽である。
pub fn 識別子として現れるか(表記: &str, 名前: &str) -> bool {
    !識別子として現れる位置一覧(表記, 名前).is_empty()
}

/// 表記の中で名前が識別子の境界で現れるバイト位置の一覧。並びは表記の中の位置の順であり、名前が空なら空である。
pub fn 識別子として現れる位置一覧(表記: &str, 名前: &str) -> Vec<usize> {
    if 名前.is_empty() {
        return Vec::new();
    }
    表記
        .match_indices(名前)
        .filter(|(位置, _)| {
            let 前が境界か = 表記[..*位置].chars().next_back().is_none_or(|文字| !識別子の文字か(文字));
            let 後ろが境界か = 表記[*位置 + 名前.len()..].chars().next().is_none_or(|文字| !識別子の文字か(文字));
            前が境界か && 後ろが境界か
        })
        .map(|(位置, _)| 位置)
        .collect()
}

/// 識別子の文字(英数字・下線・非ASCIIの文字)か。Rust の識別子の文字(XID_Continue)の上位集合であり、検査器が識別子の文字を判定する唯一の関数である。
/// 上位集合で読み違えないのは、コードだけの行に残る非ASCIIの文字が識別子の中にしか現れないためである。rustc は全角の空白と全角の括弧を拒み、それ以外の非ASCIIの空白は空白の正規形(`whitespace_form_assertion.rs`)が禁じる。
pub fn 識別子の文字か(文字: char) -> bool {
    文字.is_ascii_alphanumeric() || 文字 == '_' || !文字.is_ascii()
}

#[cfg(test)]
mod tests {
    use super::{識別子として現れるか, 識別子として現れる位置一覧};

    #[test]
    fn 境界で現れた位置だけを位置の順に答える() {
        assert_eq!(識別子として現れる位置一覧("impl 甲 { implicit impl }", "impl"), vec![0, "impl 甲 { implicit ".len()]);
        assert!(識別子として現れる位置一覧("甲", "").is_empty());
    }

    #[test]
    fn 区切りの無い日本語の識別子の途中では当たらない() {
        for 表記 in ["歩数を動かす領域", "旧歩数", "Vec<歩数の一覧>"] {
            assert!(!識別子として現れるか(表記, "歩数"), "{表記}");
        }
    }

    #[test]
    fn 英数字でない記号の文字も名前の途中なら識別子の文字として続ける() {
        for 表記 in ["T·x", "型·for", "名・前"] {
            assert!(!識別子として現れるか(表記, "T") && !識別子として現れるか(表記, "for") && !識別子として現れるか(表記, "名"), "{表記}");
        }
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn 設計オントロジーの検査器は識別子の文字をこの関数の外で判定しない() {
        let 根 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/conform/design_ontology");
        let mut 原文一覧 = vec![(根.with_extension("rs"), std::fs::read_to_string(根.with_extension("rs")).expect("親のモジュールを読む"))];
        let mut 未読 = vec![根];
        while let Some(ディレクトリ) = 未読.pop() {
            for 項目 in std::fs::read_dir(&ディレクトリ).expect("ディレクトリを読む") {
                let パス = 項目.expect("項目を読む").path();
                match パス.extension() {
                    None => 未読.push(パス),
                    Some(拡張子) if 拡張子 == "rs" => 原文一覧.push((パス.clone(), std::fs::read_to_string(&パス).expect("ソースを読む"))),
                    Some(_) => {}
                }
            }
        }
        let 判定の名前 = concat!("is_ascii", "_alphanumeric");
        for (パス, 原文) in &原文一覧 {
            for 使ってはならない名前 in [concat!("is", "_alphanumeric"), concat!("is", "_alphabetic")] {
                assert!(!原文.contains(使ってはならない名前), "{}", パス.display());
            }
            let 許す件数 = usize::from(パス.ends_with("identifier_boundary.rs"));
            assert_eq!(原文.matches(判定の名前).count(), 許す件数, "{}", パス.display());
        }
    }

    #[test]
    fn パスの区切りと括弧と参照の後ろでも当たる() {
        for 表記 in ["地点", "crate::歩行::地点", "Vec<地点>", "&mut 地点", "(甲::地点, u8)", "[地点]"] {
            assert!(識別子として現れるか(表記, "地点"), "{表記}");
        }
    }
}
