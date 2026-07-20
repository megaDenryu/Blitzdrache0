//! アニメーションクリップ: 名前・継続秒と、ジョイント添字ごとのTRSチャンネル一覧。

use blitz_math::秒;

use super::joint_channel::ジョイントアニメーションチャンネル;

/// `ジョイントチャンネル一覧`の添字はスキンの`ジョイント一覧`の添字(トポロジカル順)と対応する。
#[derive(Debug, Clone, PartialEq)]
pub struct アニメーションクリップ {
    pub 名前: String,
    pub 継続秒: 秒,
    pub ジョイントチャンネル一覧: Vec<ジョイントアニメーションチャンネル>,
}
