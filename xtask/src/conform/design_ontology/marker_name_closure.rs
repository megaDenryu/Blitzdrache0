//! 名前の言い換えの辺の一覧と、1つの名前から辺をたどって閉じた名前の並び。全ソースから1度だけ組み、受け取るのは種の名前、返すのは閉じた名前の並びである。
//! 辺は2つである。`type 別名 = 右辺;` の別名と右辺のパスの最後の名前、`use …::元 as 別名;` の別名と元の名前である。どちらも双方向に入れる。
//! 双方向にするのは、マーカーを別名の側へ書く形(`type 別名 = 規則; impl M不変データ for 別名`)で本体の型名 `規則` が漏れるためである。
//! 正規形の検査(`name_traceable_form_assertion.rs`)が右辺の先頭を裸のパスに限るため、辺は必ずたどれる。
//! 並びを辞書順で返すのは、同じ入力から常に同じ違反の説明が出るようにするためである。集合の並びの順に走査すると、報告する実装が実行のたびに変わりうる。

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use super::line_matching::パスの最後の名前;
use super::type_alias_scan::型の別名の宣言を読む;
use super::use_resolution::書き出しの行付きの取り込みの項目一覧;

#[derive(Default)]
pub struct 名前の言い換えの辺一覧 {
    名前ごとの相手一覧: HashMap<String, Vec<String>>,
}

impl 名前の言い換えの辺一覧 {
    pub fn 全ソースから組む(ソース一覧: &[(PathBuf, Vec<String>)]) -> Self {
        let mut 辺一覧 = Self::default();
        for (_, 行一覧) in ソース一覧 {
            for 行 in 行一覧 {
                if let Some(宣言) = 型の別名の宣言を読む(行.as_str()).読めた値() {
                    辺一覧.双方向へ足す(&宣言.別名, パスの最後の名前(&宣言.右辺));
                }
            }
            for 在り処 in 書き出しの行付きの取り込みの項目一覧(行一覧).項目一覧 {
                if let Some(別名) = 在り処.項目.別名.as_deref() {
                    辺一覧.双方向へ足す(別名, 在り処.項目.元の名前());
                }
            }
        }
        辺一覧
    }

    /// 種の名前から辺を双方向にたどり、変化が止まるまで閉じた名前の並び。種そのものを必ず含み、並びは辞書順である。
    pub fn 閉包(&self, 種: &str) -> Vec<String> {
        let mut 集合 = BTreeSet::from([種.to_string()]);
        let mut 未処理 = vec![種.to_string()];
        while let Some(名前) = 未処理.pop() {
            for 相手 in self.名前ごとの相手一覧.get(&名前).map_or(&[][..], Vec::as_slice) {
                if 集合.insert(相手.clone()) {
                    未処理.push(相手.clone());
                }
            }
        }
        集合.into_iter().collect()
    }

    // 2つの名前を互いの相手として足す。どちらかが空か、2つが同じなら足さない。
    fn 双方向へ足す(&mut self, 片方: &str, もう片方: &str) {
        if 片方.is_empty() || もう片方.is_empty() || 片方 == もう片方 {
            return;
        }
        for (名前, 相手) in [(片方, もう片方), (もう片方, 片方)] {
            let 相手一覧 = self.名前ごとの相手一覧.entry(名前.to_string()).or_default();
            if !相手一覧.iter().any(|登録済み| 登録済み == 相手) {
                相手一覧.push(相手.to_string());
            }
        }
    }
}
