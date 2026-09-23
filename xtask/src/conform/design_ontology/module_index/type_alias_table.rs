//! 走査範囲の型の別名(`type 名前 = 右辺;`)の表。モジュールの索引が全ソースを1度だけ走査する中で、モジュールの直下の行を足して組み、別名が型名を指しうるかを答える。実装の対象の型の表記を定義へ結び付ける工程(`implementation_binding.rs`)が使う。
//! 表はワークスペース全体で1つにする。別のクレートの `pub type 別名 = 規則;` を取り込んで `impl 変更 for 別名` と書く実装も、型名 `規則` を指しうるためである。
//! 同じ名前の別名が複数あれば右辺を全部持つ。1つだけ残すと、後に読んだ別名が先の別名を上書きし、先の別名を通した実装を黙って外すためである。
//! 別名として拾うのは、モジュールの直下(`mod 名前 { ... }` の中を含む)の `type` だけである。実装とトレイトの本体の中の関連型(`type Output = Self;`)は型の別名ではなく、
//! 関数の本体の中の `type` はその本体の中でしか見えない(局所の別名は `module_index/file_lexical_position.rs` が持つ)。

use std::collections::HashMap;

use super::super::declaration_prefix::属性と可視性を読み飛ばす;
use super::super::line_matching::{パスの最後の名前, 先頭の識別子};

#[derive(Default)]
pub struct 型の別名の表 {
    別名ごとの右辺の名前: HashMap<String, Vec<String>>, // 別名から、右辺のパスの最後の名前(型引数を除く)の一覧への対応
}

impl 型の別名の表 {
    /// モジュールの直下の1行が `type` の別名なら、表へ足す。
    pub fn モジュールの直下の行を足す(&mut self, 行: &str) {
        if let Some((別名, 右辺の名前)) = 型の別名の宣言(行) {
            self.別名ごとの右辺の名前.entry(別名).or_default().push(右辺の名前);
        }
    }

    /// 走査範囲のどこかの `type 別名 = 右辺;` の右辺の最後の名前が型名か。
    pub fn 別名が型名を指しうるか(&self, 別名: &str, 型名: &str) -> bool {
        self.別名ごとの右辺の名前.get(別名).is_some_and(|右辺一覧| 右辺一覧.iter().any(|右辺| 右辺 == 型名))
    }
}

/// `type 別名<..> = 右辺;` の行なら、別名と、右辺のパスの最後の名前の組。
pub(super) fn 型の別名の宣言(行: &str) -> Option<(String, String)> {
    let 後ろ = 属性と可視性を読み飛ばす(行).strip_prefix("type ")?.trim_start();
    let 右辺 = 後ろ.split_once('=')?.1.split(';').next().unwrap_or_default();
    Some((先頭の識別子(後ろ), パスの最後の名前(右辺.trim()).to_string()))
}
