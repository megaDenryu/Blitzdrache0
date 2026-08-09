//! 描画シーン素材: 描画対象素材の非空の一覧と、その中で動く個体の宣言。
//!
//! 動く個体の宣言をこの型が持つのは、束の読込より前に確定していなければならないためである。動く個体だけが
//! フレームスロットごとのバッファを持ち、そのバッファをディスクリプタが束の読込時に結ぶ
//! (参照: `moving_instance_declaration.rs`)。

mod moving_instance_declaration;

pub use moving_instance_declaration::動く個体の宣言;

use crate::error::{レンダラーエラー, 生成要求不一致エラー};
use crate::render_object_material::描画対象素材;

/// 空シーンをVulkan生成境界へ渡せないようにする入力。
pub struct 描画シーン素材 {
    描画対象一覧: Vec<描画対象素材>,
    動く個体一覧: Vec<動く個体の宣言>,
}

impl 描画シーン素材 {
    pub fn 生成する(先頭の描画対象: 描画対象素材, 残りの描画対象一覧: Vec<描画対象素材>) -> Self {
        let mut 描画対象一覧 = Vec::with_capacity(1 + 残りの描画対象一覧.len());
        描画対象一覧.push(先頭の描画対象);
        描画対象一覧.extend(残りの描画対象一覧);
        Self {
            描画対象一覧,
            動く個体一覧: Vec::new(),
        }
    }

    /// この束の中の1体を動く個体として宣言する。素材の持たない描画対象または個体を指す宣言は型付きエラーで拒む。
    /// 拒まずに受けると、束の読込がその個体の読込時の内容を持てないまま毎フレームの書き込み先だけを確保する。
    pub fn 動く個体を宣言する(&mut self, 宣言: 動く個体の宣言) -> Result<(), レンダラーエラー> {
        let 範囲外 = 生成要求不一致エラー::動く個体の宣言が素材の範囲外 {
            描画対象添字: 宣言.描画対象添字,
            個体添字: 宣言.個体添字,
        };
        let Some(描画対象) = self.描画対象一覧.get(宣言.描画対象添字) else {
            return Err(範囲外.into());
        };
        let 個体数 = u32::try_from(描画対象.個体変換一覧().len()).map_err(|_| 範囲外)?;
        if 宣言.個体添字 >= 個体数 {
            return Err(範囲外.into());
        }
        self.動く個体一覧.push(宣言);
        Ok(())
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

    pub(crate) fn 動く個体一覧(&self) -> &[動く個体の宣言] {
        &self.動く個体一覧
    }
}
