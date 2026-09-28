//! トレイトの実装の本体の関数の署名を、関連型を含めて普通のRustの書き方のまま読むことと、標準ライブラリのトレイトの実装の関数を明示して外すことを固定する回帰試験。
//! 事実の組み方と関係の表記の読み方は親の試験の関数を使う。

use super::{抽出する, 欠落無しで関係を読む};
use crate::設計関係のモデル::抽出できなかった理由;

// 反証: `Self` を先に実装の対象の型へ置き換えると `道::出力` になり、型の関連項目として読めない表記に数え、関係を落とした欠落が出る。
// 関連型の宣言を関数より後ろに書いても読むことも固定する。
#[test]
fn 実装の本体の関数に書いた自分の型の関連型をその実装の関連型の宣言の右辺として読む() {
    let 原文 = "pub struct 地点 {\n    番号: u8,\n}\n\npub struct 道 {\n    今: u8,\n}\n\npub trait 口 {\n    type 出力;\n\n    fn 出す(&self) -> Self::出力;\n}\n\nimpl 口 for 道 {\n    fn 出す(&self) -> Self::出力 {\n        todo!()\n    }\n\n    type 出力 = 地点;\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.iter().any(|表記| 表記.contains("出す 返す blitz_game::試験::地点")), "{表記一覧:?}");
}

// 反証: 実装の本体に宣言の無い関連型を黙って外すと、関係を落としたことが見えなくなる。保証範囲の外として欠けに数える。
#[test]
fn 実装の本体に宣言の無い自分の型の関連型は読めない表記として数える() {
    let 原文 = "pub struct 道 {\n    今: u8,\n}\n\nimpl 道 {\n    pub fn 出す(&self) -> Self::出力 {\n        todo!()\n    }\n}\n";
    let 結果 = 抽出する(原文);
    assert!(
        結果.抽出できなかった行一覧.iter().any(|行| matches!(行.理由, 抽出できなかった理由::関数の署名の型の表記を読めない { .. })),
        "{:?}",
        結果.抽出できなかった行一覧
    );
}

// 反証: 標準ライブラリのトレイトの実装の関数を署名の事実にすると、`clone` や `add` が状態を受けて返す処理の宣言の法則の前件に当たる。
// 修飾して書いたトレイト・`use` で取り込んだトレイト・プレリュードのトレイトのどれも、明示して除外した件数に数え、欠落にしない。
#[test]
fn 標準ライブラリのトレイトの実装の関数は署名を読まず明示して除外する() {
    let 原文 = "use std::fmt;\nuse std::ops::Add;\n\npub struct 居場所 {\n    番号: u8,\n}\n\nimpl Clone for 居場所 {\n    fn clone(&self) -> Self {\n        todo!()\n    }\n}\n\nimpl Add for 居場所 {\n    type Output = Self;\n\n    fn add(self, 他: Self) -> Self::Output {\n        todo!()\n    }\n}\n\nimpl fmt::Display for 居場所 {\n    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {\n        todo!()\n    }\n}\n\nimpl std::iter::Iterator for 居場所 {\n    type Item = u8;\n\n    fn next(&mut self) -> Option<Self::Item> {\n        todo!()\n    }\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(!表記一覧.iter().any(|表記| ["clone", "add", "fmt", "next"].iter().any(|名前| 表記.contains(&format!("::{名前}")))), "{表記一覧:?}");
    let 除外の件数 = 抽出する(原文).抽出できなかった行一覧.iter().filter(|行| 行.理由.種別の呼び名() == "標準ライブラリのトレイトの実装の関数である").count();
    assert_eq!(除外の件数, 4);
}

// 反証: トレイトの実装の関数を一律に除外すると、このクレートで宣言したトレイトの実装の関数まで署名を失う。
#[test]
fn このクレートのトレイトの実装の関数は署名を読む() {
    let 原文 = "pub struct 居場所 {\n    番号: u8,\n}\n\npub trait 進められる {\n    fn 進めた(self) -> Self;\n}\n\nimpl 進められる for 居場所 {\n    fn 進めた(self) -> Self {\n        todo!()\n    }\n}\n";
    let 表記一覧 = 欠落無しで関係を読む(原文);
    assert!(表記一覧.iter().any(|表記| 表記.contains("居場所::進められる::進めた そのまま返す blitz_game::試験::居場所")), "{表記一覧:?}");
}
