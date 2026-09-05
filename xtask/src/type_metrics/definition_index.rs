//! 走査で見つけた型定義の索引と、implブロックを定義へ引き当てる判断。引き当てた結果の型は`impl_attribution`が持つ。
//!
//! 型を名前と定義ファイルの組で識別すると、implブロックだけが載っている行から所在が決まらない。
//! 同じ名前の型が別の場所に複数あるとき、そのimplがどちらの型のものかを決める必要がある。
//! 判断はモジュールの木の近さで行う。inherent implは定義と同じクレートにしか置けず、実際には
//! 定義と同じモジュールの木の中に置かれるためである。近さが同点の定義が2つ以上あるときは、
//! 黙ってどちらかへ寄せず、引き当てられなかったこととして呼び出し側へ返す。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::impl_attribution::{定義の候補が複数ある実装ブロック, 実装ブロックの引き当て};
use super::observation::観測;
use super::type_location::型の所在;

pub struct 定義の索引 {
    型名ごとの定義の所在: BTreeMap<String, Vec<型の所在>>,
}

impl 定義の索引 {
    pub fn ファイル別の観測から生成する(ファイル別観測: &[(PathBuf, Vec<観測>)]) -> Self {
        let mut 型名ごとの定義の所在: BTreeMap<String, Vec<型の所在>> = BTreeMap::new();
        for (パス, 観測一覧) in ファイル別観測 {
            for 観測 in 観測一覧 {
                let 観測::型定義 { 型名, .. } = 観測 else { continue };
                let 所在 = 型の所在::走査したファイルから生成する(パス, 型名);
                let 一覧 = 型名ごとの定義の所在.entry(型名.clone()).or_default();
                if !一覧.contains(&所在) {
                    一覧.push(所在);
                }
            }
        }
        Self {
            型名ごとの定義の所在
        }
    }

    /// 定義が走査に1つも現れない型のimplブロックは、そのimplブロックのファイルを所在にする。
    /// 型別名・外部の型・マクロが作る型がこれに当たり、読み手が開ける唯一のファイルがそこだからである。
    pub fn 実装ブロックの所在を引き当てる(
        &self, 実装ブロックのファイル: &Path, 型名: &str
    ) -> 実装ブロックの引き当て {
        let 候補一覧 = self.型名ごとの定義の所在.get(型名).map_or(&[][..], Vec::as_slice);
        let Some(最も近い) = 候補一覧
            .iter()
            .max_by_key(|所在| 所在.実装ブロックのファイルへの近さ(実装ブロックのファイル))
        else {
            return 実装ブロックの引き当て::所在が決まった(型の所在::走査したファイルから生成する(実装ブロックのファイル, 型名));
        };
        let 最も近い近さ = 最も近い.実装ブロックのファイルへの近さ(実装ブロックのファイル);
        let 同点の候補一覧: Vec<型の所在> = 候補一覧
            .iter()
            .filter(|所在| 所在.実装ブロックのファイルへの近さ(実装ブロックのファイル) == 最も近い近さ)
            .cloned()
            .collect();
        if 同点の候補一覧.len() > 1 {
            return 実装ブロックの引き当て::定義の候補を1つに絞れない(定義の候補が複数ある実装ブロック::生成する(
                実装ブロックのファイル.to_path_buf(),
                型名.to_string(),
                同点の候補一覧,
            ));
        }
        実装ブロックの引き当て::所在が決まった(最も近い.clone())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::type_metrics::declaration_amount::宣言の分量;

    fn 定義(型名: &str) -> 観測 {
        観測::型定義 {
            型名: 型名.to_string(),
            分量: 宣言の分量::構造体のフィールド数(1),
        }
    }

    fn 索引(ファイル一覧: &[&str]) -> 定義の索引 {
        let 観測: Vec<(PathBuf, Vec<観測>)> = ファイル一覧
            .iter()
            .map(|ファイル| (PathBuf::from(ファイル), vec![定義("設定")]))
            .collect();
        定義の索引::ファイル別の観測から生成する(&観測)
    }

    fn 決まった所在(索引: &定義の索引, 実装ブロックのファイル: &str) -> String {
        match 索引.実装ブロックの所在を引き当てる(Path::new(実装ブロックのファイル), "設定") {
            実装ブロックの引き当て::所在が決まった(所在) => 所在.to_string(),
            実装ブロックの引き当て::定義の候補を1つに絞れない(実装ブロック) => {
                panic!("絞れなかった: {}", 実装ブロック.候補の綴り())
            }
        }
    }

    #[test]
    fn 同じ名前の定義が2つあればモジュールの木が近い方へ引き当てる() {
        let 索引 = 索引(&["a/src/near/def.rs", "a/src/far/def.rs"]);
        assert_eq!(決まった所在(&索引, "a/src/near/use.rs"), "a/src/near/def.rs::設定");
        assert_eq!(決まった所在(&索引, "a/src/far/use.rs"), "a/src/far/def.rs::設定");
    }

    #[test]
    fn 定義が走査に無ければ実装ブロックのファイルを所在にする() {
        assert_eq!(決まった所在(&索引(&[]), "a/src/alias.rs"), "a/src/alias.rs::設定");
    }

    #[test]
    fn 近さが同点なら引き当てず候補を返す() {
        let 索引 = 索引(&["a/src/left/def.rs", "a/src/right/def.rs"]);
        let 引き当て = 索引.実装ブロックの所在を引き当てる(Path::new("a/src/lib.rs"), "設定");
        let 実装ブロックの引き当て::定義の候補を1つに絞れない(実装ブロック) = 引き当て else {
            panic!("同点なのに1つへ絞った");
        };
        assert_eq!(実装ブロック.候補の綴り(), "a/src/left/def.rs::設定 / a/src/right/def.rs::設定");
    }
}
