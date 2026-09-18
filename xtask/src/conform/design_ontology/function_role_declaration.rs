//! 関数役割の宣言の探索。呼び出し連鎖の中の独立した工程であり、受け取るのは全ソース、
//! 返すのは `const _: M遷移関数<...> = 右辺;` の形の宣言の一覧(関数型・右辺・場所)である。
//! 右辺が確認関数で始まるかの判定は宣言自身が持つ。

use std::path::PathBuf;

use super::syntax_patterns::{オントロジー関数型, 構文パターン};

/// `const _: 関数型<...> = 右辺;` の1件。行番号は1始まりであり、違反の報告がこの行を指す。
pub struct 関数役割の宣言 {
    pub 関数: オントロジー関数型,
    pub 右辺: String,
    pub パス: PathBuf,
    pub 行番号: usize,
}

impl 関数役割の宣言 {
    /// 右辺が、その関数型に対応する確認関数の呼び出しで始まるか。`blitz_design::` のようなパスの修飾は名前の前に置いてよい。
    pub fn 確認関数を通っているか(&self) -> bool {
        let Some((呼び出し先, _)) = self.右辺.split_once('(') else {
            return false;
        };
        呼び出し先.rsplit("::").next().unwrap_or_default() == self.関数.確認関数の名前()
    }
}

pub fn 関数役割の宣言を探す(ソース一覧: &[(PathBuf, Vec<String>)]) -> Vec<関数役割の宣言> {
    let mut 宣言一覧 = Vec::new();
    for (パス, 行一覧) in ソース一覧 {
        for (添字, 行) in 行一覧.iter().enumerate() {
            let Some(関数) = オントロジー関数型::全部().into_iter().find(|関数| 行.trim().starts_with(&構文パターン::静的型宣言の接頭辞(*関数))) else {
                continue;
            };
            let 右辺 = 右辺(行一覧, 添字);
            宣言一覧.push(関数役割の宣言 {
                関数,
                右辺,
                パス: パス.clone(),
                行番号: 添字 + 1,
            });
        }
    }
    宣言一覧
}

// 宣言の `=` より後ろ。`=` が無いか後ろが空なら次の行を右辺とし、それも無ければ空である。
fn 右辺(行一覧: &[String], 添字: usize) -> String {
    let 同じ行 = 行一覧[添字].split_once('=').map(|(_, 後ろ)| 後ろ.trim()).unwrap_or_default();
    if !同じ行.is_empty() {
        return 同じ行.to_string();
    }
    行一覧.get(添字 + 1).map(|行| 行.trim().to_string()).unwrap_or_default()
}
