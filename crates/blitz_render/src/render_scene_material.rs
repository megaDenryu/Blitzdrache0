//! 描画シーン素材: 描画対象素材の非空の一覧。

use crate::render_object_material::描画対象素材;

/// 空シーンをVulkan生成境界へ渡せないようにする入力。
pub struct 描画シーン素材 {
    描画対象一覧: Vec<描画対象素材>,
}

impl 描画シーン素材 {
    pub fn 生成する(先頭の描画対象: 描画対象素材, 残りの描画対象一覧: Vec<描画対象素材>) -> Self {
        let mut 描画対象一覧 = Vec::with_capacity(1 + 残りの描画対象一覧.len());
        描画対象一覧.push(先頭の描画対象);
        描画対象一覧.extend(残りの描画対象一覧);
        Self { 描画対象一覧 }
    }

    pub fn 先頭の描画対象(&self) -> &描画対象素材 {
        match self.描画対象一覧.first() {
            Some(描画対象) => 描画対象,
            None => panic!("描画シーン素材は1つ以上の描画対象を持つ不変条件に違反した"),
        }
    }

    pub fn 描画対象数(&self) -> usize {
        self.描画対象一覧.len()
    }

    pub(crate) fn 描画対象一覧(&self) -> &[描画対象素材] {
        &self.描画対象一覧
    }
}
