//! キーフレーム間の補間方式。CUBICSPLINEは判断42によりM8スコープ外(型として存在しない)。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 補間種別 {
    ステップ,
    線形,
}
