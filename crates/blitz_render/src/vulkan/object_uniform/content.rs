//! 描画対象ごとの変換とマテリアル係数。先頭は個体変換1件と同じ配置であり、単一個体の対象はこの先頭を個体変換として読む。

use super::super::instance_transform::content::個体変換内容;

pub(super) struct 描画対象ユニフォーム内容 {
    pub(super) 変換: 個体変換内容,
    pub(super) ベースカラー係数: [f32; 4],
    pub(super) 金属粗さ係数: [f32; 2],
}
