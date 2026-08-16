//! クォータニオン生成の失敗。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! 注意: blitz_mathはglamのみに依存する方針(数学DDD最小セットの外部依存最小化)のため
//! thiserrorを使わず`std::error::Error`を手動実装する。

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum クォータニオンエラー {
    ゼロ長,
    /// 3軸の組が右手系をなしていない。左手系(鏡像)は回転では重ねられない。
    軸の組が右手系でない {
        行列式: f32,
    },
}

impl fmt::Display for クォータニオンエラー {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ゼロ長 => write!(f, "回転を表すクォータニオンの長さがゼロに近い"),
            Self::軸の組が右手系でない { 行列式 } => {
                write!(f, "3軸の組が右手系でない(行列式{行列式}。右手系なら1になる)")
            }
        }
    }
}

impl std::error::Error for クォータニオンエラー {}
