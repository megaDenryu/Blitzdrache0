//! 掃引の答え: 世界の形を尋ねる口が返す、値(掃引が最初に触れる面)と完全性の対。
//! 世界側の世界問い合わせ結果を1対1で写した形であり、2軸を1つへ潰さない。
//! 参照: `_doc/設計/キャラクターの移動とカメラ.md`「判断3」

use crate::sweep_completeness::掃引の完全性;
use crate::sweep_hit::掃引が最初に触れる面;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 掃引の答え {
    面: 掃引が最初に触れる面,
    完全性: 掃引の完全性,
}

impl 掃引の答え {
    pub fn 生成する(面: 掃引が最初に触れる面, 完全性: 掃引の完全性) -> Self {
        Self { 面, 完全性 }
    }

    pub fn 面(&self) -> 掃引が最初に触れる面 {
        self.面
    }

    pub fn 完全性(&self) -> 掃引の完全性 {
        self.完全性
    }
}
