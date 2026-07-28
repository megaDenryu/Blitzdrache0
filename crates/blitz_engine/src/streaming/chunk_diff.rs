//! 必要集合の反映で発生した読込と解除、および準備完了後の行先。

use crate::チャンク座標;

use super::chunk_request::チャンク要求;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct チャンク集合差分 {
    読込要求一覧: Vec<チャンク要求>,
    解除要求一覧: Vec<チャンク座標>,
}

impl チャンク集合差分 {
    pub(super) fn 生成する(読込要求一覧: Vec<チャンク要求>, 解除要求一覧: Vec<チャンク座標>) -> Self {
        Self {
            読込要求一覧, 解除要求一覧
        }
    }

    pub fn 読込要求一覧(&self) -> &[チャンク要求] {
        &self.読込要求一覧
    }

    pub fn 解除要求一覧(&self) -> &[チャンク座標] {
        &self.解除要求一覧
    }

    /// 上限超過で退避させた座標を解除要求へ加える。解除の経路を1本に保ち、呼出し側が不要化と退避を見分けずに同じ後始末をできるようにする。
    /// 不要化の分は座標順、退避の分は退避した順で並ぶ。どちらも決定的であるため、連結した列も同じ入力から同じ順になる。
    pub(super) fn 解除要求を足す(&mut self, 追加: &[チャンク座標]) {
        self.解除要求一覧.extend_from_slice(追加);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 準備完了結果 {
    GPU転送待ち,
    CPUデータ破棄,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPU転送完了結果 {
    フレーム反映待ち,
    GPU資源解除待ち,
}

/// GPU資源の実破棄が報告されたときの台帳側の帰結。台帳に無い座標の報告は、リセットで台帳を空にした後に
/// 実破棄が届いた正常な後始末であり、エラーではないことを型で表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 解除報告結果 {
    台帳から削除した,
    台帳に無かった,
}
