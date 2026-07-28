//! そのフレームにシーンパスが描く個体の選択。可視判定はエンジン側の純ロジックが行い、レンダラーは結果だけを受け取る。
//! 選択を持たない描画対象は全個体を描く。通常メッシュと地形、およびシャドウパスがこれに当たる。
//! カメラ視錐台で絞った可視集合をシャドウパスへ流用してはならないという不変条件を、この入力がシーンパス専用であることで守る
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「可視判定」)。

mod error;

pub use error::可視ID列エラー;

use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;

/// 1つの描画対象について、可視ID列の中でその対象が使う区間を指す。IDそのものを所有しないのは、
/// 全群の可視IDを1本の列へ連ねて毎フレームのヒープ確保をなくすためである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 可視個体選択 {
    束id: 描画束ID,
    描画対象添字: usize,
    可視id開始: usize,
    可視数: usize,
}

impl 可視個体選択 {
    pub fn 生成する(束id: 描画束ID, 描画対象添字: usize, 可視id開始: usize, 可視数: usize) -> Self {
        Self {
            束id,
            描画対象添字,
            可視id開始,
            可視数,
        }
    }

    pub(crate) fn 束id(&self) -> 描画束ID {
        self.束id
    }

    pub(crate) fn 描画対象添字(&self) -> usize {
        self.描画対象添字
    }

    pub(crate) fn 終端(&self) -> Option<usize> {
        self.可視id開始.checked_add(self.可視数)
    }

    pub(crate) fn 開始(&self) -> usize {
        self.可視id開始
    }
}

/// 選択の一覧と、その選択が指す可視ID列の本体。2つを1つの型にまとめるのは、区間が本体の内側に収まることを
/// 生成の時点で確かめ、以降の引き当てが範囲外を持ち得ないようにするためである。
#[derive(Debug, Clone, Copy)]
pub struct 可視個体選択一覧<'a> {
    選択一覧: &'a [可視個体選択],
    可視id列: &'a [u32],
}

impl<'a> 可視個体選択一覧<'a> {
    pub fn 生成する(選択一覧: &'a [可視個体選択], 可視id列: &'a [u32]) -> Result<Self, レンダラーエラー> {
        for 選択 in 選択一覧 {
            let 収まる = 選択.終端().is_some_and(|終端| 終端 <= 可視id列.len());
            if !収まる {
                return Err(可視ID列エラー::区間が範囲外 {
                    開始: 選択.開始(),
                    列の長さ: 可視id列.len(),
                }
                .into());
            }
        }
        Ok(Self { 選択一覧, 可視id列 })
    }

    /// 可視判定を行わないフレームの入力。すべての描画対象が全個体を描く。
    pub fn 空() -> Self {
        Self {
            選択一覧: &[],
            可視id列: &[],
        }
    }

    /// 該当する選択が無ければ`None`を返す。呼び出し元はそれを「この対象は可視判定の対象外であり全個体を描く」と読む。
    /// 走査が線形なのは、選択の件数が常駐する群の数であり、束ごとの描画対象数と同じ桁に収まるためである。
    pub(crate) fn 引く(&self, 束id: 描画束ID, 描画対象添字: usize) -> Option<&'a [u32]> {
        let 選択 = self
            .選択一覧
            .iter()
            .find(|選択| 選択.束id() == 束id && 選択.描画対象添字() == 描画対象添字)?;
        let 終端 = 選択.終端()?;
        self.可視id列.get(選択.開始()..終端)
    }
}
