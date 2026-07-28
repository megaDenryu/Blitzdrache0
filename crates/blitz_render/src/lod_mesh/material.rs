//! メッシュ素材: 1つの詳細段が持つ頂点一覧とインデックス一覧。

use crate::vertex::頂点;

pub struct メッシュ素材 {
    頂点一覧: Vec<頂点>,
    インデックス一覧: Vec<u32>,
}

impl メッシュ素材 {
    pub fn 生成する(頂点一覧: Vec<頂点>, インデックス一覧: Vec<u32>) -> Self {
        Self {
            頂点一覧, インデックス一覧
        }
    }

    pub fn 頂点一覧(&self) -> &[頂点] {
        &self.頂点一覧
    }

    pub(crate) fn インデックス一覧(&self) -> &[u32] {
        &self.インデックス一覧
    }
}
