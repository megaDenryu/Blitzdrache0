//! 名前が当たった自己変更の実装1件。マーカーを名乗る型の名前が対象の表記に識別子として現れ、しかも自己変更の根拠を持つ実装のことである。
//! 名前で実装を集めるのは過大近似であるため、同じ名前の別の型の実装がここへ現れうる。その1件は台帳(`name_match_exclusion_ledger.rs`)へ理由を書いて除く。

use std::path::PathBuf;

use super::self_modification_evidence::自己変更の根拠;

pub struct 名前が当たった自己変更の実装 {
    pub マーカーの名前: String,
    pub パス: PathBuf,
    pub 見出し: String, // 本体を開く `{` の手前までを空白の並びを1つに揃えた表記。台帳の行と照らす鍵である
    pub 根拠: 自己変更の根拠,
}
