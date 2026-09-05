//! implブロックが実装対象として綴った経路。`設定`・`self::設定`・`super::solver::設定`・`crate::far::設定`の
//! ように書かれ、前置きの段の並びと型名に分けて持つ。
//!
//! 経路を捨てて型名だけを残すと、同じ名前の定義が複数あるとき、どの定義のimplかをファイルの近さで推測する
//! ほかなくなる。ファイルの近さは根拠にならない。inherent implに要るのは定義と同じクレートにあることだけで
//! あり、離れたモジュールのファイルに置いてよいためである。経路をここへ残し、解決できた場合だけ定義を確定する。

use std::path::Path;

use crate::rust_module::{モジュールのディレクトリ, モジュールの位置};

const 段の区切り: &str = "::";
const クレートの根を指す段: &str = "crate";
const 自分のモジュールを指す段: &str = "self";
const 上位のモジュールを指す段: &str = "super";

pub struct 自己型の経路 {
    前置きの段一覧: Vec<String>,
    型名: String,
}

impl 自己型の経路 {
    pub fn 綴りから生成する(綴り: &str) -> Self {
        let mut 段一覧: Vec<String> = 綴り.split(段の区切り).map(str::to_string).collect();
        let 型名 = 段一覧.pop().unwrap_or_default();
        Self {
            前置きの段一覧: 段一覧,
            型名,
        }
    }

    pub fn 型名(&self) -> &str {
        &self.型名
    }

    /// 型名の前に段を書いているか。書いてあるなら、その経路が指す先だけが実装対象であり、
    /// そのファイルの`use`が持ち込んだ同じ名前は実装対象ではない。
    pub fn 経路を明示しているか(&self) -> bool {
        !self.前置きの段一覧.is_empty()
    }

    /// この経路が指すモジュールの本体になりうるファイルの綴り。`crate`・`super`・`self`から始まらない経路は
    /// `use`が持ち込んだ短い名前か外部のクレートの名前であり、経路だけでは場所が決まらないため空を返す。
    pub fn 定義ファイルの候補一覧(&self, 経路を書いたファイル: &Path) -> Vec<String> {
        let 位置 = モジュールの位置::定義ファイルから生成する(経路を書いたファイル);
        let Some((起点, 残りの段一覧)) = self.起点と残りの段(&位置) else {
            return Vec::new();
        };
        let 到達 = 残りの段一覧.iter().fold(起点, |ディレクトリ, 段| ディレクトリ.子のモジュールへ進む(段));
        到達.本体になりうるファイルの綴り一覧()
    }

    fn 起点と残りの段(&self, 位置: &モジュールの位置) -> Option<(モジュールのディレクトリ, &[String])> {
        let [先頭, 残り @ ..] = self.前置きの段一覧.as_slice() else {
            return None;
        };
        if 先頭.as_str() == クレートの根を指す段 {
            return Some((位置.クレートの根のディレクトリ()?, 残り));
        }
        if 先頭.as_str() == 自分のモジュールを指す段 {
            return Some((位置.自分のモジュールのディレクトリ(), 残り));
        }
        if 先頭.as_str() != 上位のモジュールを指す段 {
            return None;
        }
        let 上位の段数 = self
            .前置きの段一覧
            .iter()
            .take_while(|段| 段.as_str() == 上位のモジュールを指す段)
            .count();
        Some((
            位置.自分のモジュールのディレクトリ().上位のモジュールへ戻る(上位の段数)?,
            self.前置きの段一覧.get(上位の段数..)?,
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 候補(綴り: &str, ファイル: &str) -> Vec<String> {
        自己型の経路::綴りから生成する(綴り).定義ファイルの候補一覧(Path::new(ファイル))
    }

    #[test]
    fn 短い名前だけの経路は場所を決められない() {
        let 経路 = 自己型の経路::綴りから生成する("設定");
        assert_eq!(経路.型名(), "設定");
        assert!(!経路.経路を明示しているか());
        assert!(候補("設定", "a/src/near/impl.rs").is_empty());
    }

    #[test]
    fn クレートの根からの経路はモジュールの本体のファイルを指す() {
        assert!(候補("crate::far::設定", "a/src/near/impl.rs").contains(&"a/src/far.rs".to_string()));
        assert!(候補("crate::far::設定", "a/src/near/impl.rs").contains(&"a/src/far/mod.rs".to_string()));
    }

    #[test]
    fn 上位からの経路は連なった段の分だけ遡る() {
        assert!(候補("super::sweep_solver::求解", "a/src/triangle/dispatch.rs").contains(&"a/src/triangle/sweep_solver.rs".to_string()));
        assert!(候補("super::super::設定", "a/src/near/deep/impl.rs").contains(&"a/src/near.rs".to_string()));
    }

    #[test]
    fn 外部のクレートから始まる経路は場所を決められない() {
        assert!(候補("ash::vk::設定", "a/src/near/impl.rs").is_empty());
    }
}
