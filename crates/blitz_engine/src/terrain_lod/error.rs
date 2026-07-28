//! 地形LOD選択の型付きエラー。設定値の検証だけが失敗しうるため、この1つの語彙で表す。

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum 地形LODエラー {
    #[error("基準距離が正の有限メートルでない")]
    基準距離不正,
    #[error("ヒステリシス幅が0以上の有限メートルでない")]
    ヒステリシス幅不正,
    #[error("ヒステリシス幅が基準距離の半分以上である")]
    ヒステリシス幅過大,
    #[error("チャンクの一辺が正の有限メートルでない")]
    一辺不正,
}
