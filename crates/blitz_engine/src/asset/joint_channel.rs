//! 1ジョイント分のTRSチャンネル。各成分は無指定(恒等)を型で表すためOption。

use super::keyframe_channel::チャンネル;

/// 平行移動・スケールは3成分(xyz)、回転は4成分(クォータニオンxyzw)のキーフレーム列。
/// いずれも`None`ならそのジョイントはその成分について恒等(判断43)。
#[derive(Debug, Clone, PartialEq)]
pub struct ジョイントアニメーションチャンネル {
    pub 平行移動: Option<チャンネル<[f32; 3]>>,
    pub 回転: Option<チャンネル<[f32; 4]>>,
    pub スケール: Option<チャンネル<[f32; 3]>>,
}

impl ジョイントアニメーションチャンネル {
    /// どの成分も持たない(全チャンネル恒等)状態を作る。
    pub fn 空を作る() -> Self {
        Self {
            平行移動: None,
            回転: None,
            スケール: None,
        }
    }
}
