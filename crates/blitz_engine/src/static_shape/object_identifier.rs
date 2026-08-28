//! 衝突対象の安定識別子: チャンクの中で1つの静的な衝突対象を指す綴り。
//!
//! 番号でなく綴りで持つのは、問い合わせの答えを制作時のデータへ辿れるようにするためである。綴りの由来は建物をチャンクへ
//! 置いた配置の識別子であり、エディターが保存した正本の値をそのまま運ぶ。焼き直しで並びが変わっても綴りは変わらない。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断10: 衝突形状が宣言できる属性」

use std::fmt;

use super::error::静的物理形状エラー;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 衝突対象の安定識別子 {
    綴り: String,
}

impl 衝突対象の安定識別子 {
    /// 空の綴りを拒む。前後の空白は落とす。
    pub fn 生成する(綴り: &str) -> Result<Self, 静的物理形状エラー> {
        let 整えた綴り = 綴り.trim();
        if 整えた綴り.is_empty() {
            return Err(静的物理形状エラー::安定識別子が空);
        }
        Ok(Self {
            綴り: 整えた綴り.to_string(),
        })
    }

    pub fn 綴り(&self) -> &str {
        &self.綴り
    }
}

impl fmt::Display for 衝突対象の安定識別子 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(出力, "{}", self.綴り)
    }
}
