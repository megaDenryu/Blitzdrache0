//! 質量の単位型を生成するときの失敗。負と非有限を拒む検査は`キログラム`と`逆キログラム`が共有する。

use std::fmt;

/// 質量または逆質量の生成が拒んだ入力。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum 質量エラー {
    /// 入力が有限値でない(非数・無限大)。
    非有限値 { 値: f32 },
    /// 入力が負である。質量も逆質量も0以上でなければならない。
    負の値 { 値: f32 },
}

impl fmt::Display for 質量エラー {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::非有限値 { 値 } => write!(formatter, "質量の量{値}が有限値でない"),
            Self::負の値 { 値 } => write!(formatter, "質量の量{値}が負である"),
        }
    }
}

impl std::error::Error for 質量エラー {}

/// 負と非有限を拒み、0以上の有限値だけを通す。
pub(super) fn 質量の量として検査する(値: f32) -> Result<f32, 質量エラー> {
    if !値.is_finite() {
        return Err(質量エラー::非有限値 { 値 });
    }
    if 値 < 0.0 {
        return Err(質量エラー::負の値 { 値 });
    }
    Ok(値)
}
