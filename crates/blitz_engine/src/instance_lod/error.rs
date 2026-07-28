//! 個体別LODの選択設定が拒む理由。段の数は原型が保証するため、ここが見るのは距離の刻みと帯の幅だけである。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum 個体LODエラー {
    #[error("基準距離が正の有限メートルでない")]
    基準距離不正,
    #[error("ヒステリシス幅が0以上の有限メートルでない")]
    ヒステリシス幅不正,
    #[error("ヒステリシス幅が基準距離の半分以上である")]
    ヒステリシス幅過大,
}
