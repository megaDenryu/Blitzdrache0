//! 段閾値帯の検証が拒む理由。距離の刻みと帯の幅の2つだけが検証の対象である。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum 段閾値エラー {
    #[error("基準距離が正の有限値でない")]
    基準距離不正,
    #[error("ヒステリシス幅が0以上の有限値でない")]
    ヒステリシス幅不正,
    #[error("ヒステリシス幅が基準距離の半分以上である")]
    ヒステリシス幅過大,
}
