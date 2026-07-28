//! 1フレームの描画発行の内訳。シーンパスとシャドウパスを別々に持つ。
//! 2つのパスを分けて数えるのは、シーンパスがカメラ視錐台、シャドウパスがライト視錐台という別々の視錐台で個体を絞り、
//! 一方の可視集合を他方へ流用しないという不変条件を数字で示すためである
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」)。
//! パス1つぶんの数は`pass_issue`、LOD段ごとの個体数は`stage_counts`にある。

mod pass_issue;
mod stage_counts;

pub use pass_issue::パス別描画発行;
pub use stage_counts::段別個体数;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 描画発行内訳 {
    シーン: パス別描画発行,
    シャドウ: パス別描画発行,
}

impl 描画発行内訳 {
    pub(crate) fn 生成する(シーン: パス別描画発行, シャドウ: パス別描画発行) -> Self {
        Self { シーン, シャドウ }
    }

    pub fn シーン(&self) -> &パス別描画発行 {
        &self.シーン
    }

    pub fn シャドウ(&self) -> &パス別描画発行 {
        &self.シャドウ
    }
}
