//! 1つのファイルの中で読んだ `impl` の1件。所在(ファイルのパスとコードだけの行の一覧と見出しの行番号)と見出しを持つ。
//! その所在から答えられる問いのうち、書いた名前が `use` の別名を通しているかをここで答える。対象の型を定義へ結び付ける問いは `implementation_binding.rs`、
//! トレイトが走査範囲の外かの問いは `implemented_trait.rs` が答える。

use std::path::Path;

use super::impl_header::実装の見出し;
use super::module_index::モジュールの索引;
use super::module_path::モジュールパス;
use super::use_resolution::取り込みの項目;

pub struct 読んだ実装<'a> {
    pub パス: &'a Path,
    pub 行一覧: &'a [String],
    pub 行番号: usize, // 見出しの行の1始まりの番号。違反の報告がこの行を指す
    pub 見出し: 実装の見出し,
}

impl 読んだ実装<'_> {
    /// 書いた名前(修飾があれば `修飾::名前`)が `use 元 as 名前` の別名を通しているなら、元のパスの最後の名前。
    /// このファイルの取り込みと、取り込み元のモジュール(修飾があれば修飾が指しうるモジュール、無ければ明示の取り込みの取り込み元か、それが無ければ glob の取り込みの取り込み元)の取り込みを1段たどる。
    /// 再公開の `pub use … as` もこれで読む。
    pub fn 別名の元の名前<'b>(&self, 索引: &'b モジュールの索引<'_>, 修飾: Option<&str>, 名前: &str) -> Option<&'b str> {
        let 自分 = モジュールパス::ファイルのパスから求める(self.パス);
        let 取り込み元一覧: Vec<&str> = match 修飾 {
            Some(修飾) => vec![修飾],
            None => {
                if let Some(元の名前) = 索引.別名の元の名前(&自分, 名前) {
                    return Some(元の名前);
                }
                let 項目一覧 = 索引.取り込みの項目一覧(&自分);
                match 項目一覧.iter().find(|項目| 項目.別名.is_none() && 項目.元の名前() == 名前) {
                    Some(項目) => vec![項目.親のパス()],
                    None => 項目一覧.iter().filter(|項目| 項目.全部を取り込むか()).map(取り込みの項目::親のパス).collect(),
                }
            }
        };
        取り込み元一覧
            .iter()
            .flat_map(|取り込み元| 自分.書かれたパスが指しうるモジュール一覧(取り込み元))
            .find_map(|モジュール| 索引.別名の元の名前(&モジュール, 名前))
    }
}
