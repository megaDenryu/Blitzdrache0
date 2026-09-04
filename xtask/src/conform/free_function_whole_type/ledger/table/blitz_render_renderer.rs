//! `blitz_render`の描画の組み立て層の未是正の自由関数の一覧。`レンダラー`と、生成要求と受け皿を丸ごと受け取るものが並ぶ。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 20] = [
    未是正の自由関数::生成する("draw_dispatch/work_area_fill.rs", "積む", "レンダラー"),
    未是正の自由関数::生成する("draw_execute/acquire.rs", "取得して束ねる", "レンダラー"),
    未是正の自由関数::生成する("draw_execute/acquire.rs", "実行する", "レンダラー"),
    未是正の自由関数::生成する("draw_execute/prepare.rs", "実行する", "レンダラー"),
    未是正の自由関数::生成する("draw_execute/prepare.rs", "資源表世代のフェンス通過を記録する", "レンダラー"),
    未是正の自由関数::生成する("draw_execute/submit.rs", "実行する", "レンダラー"),
    未是正の自由関数::生成する("draw_stage_resources/create/optional_stages.rs", "大気の上へ積む", "任意段階の資源"),
    未是正の自由関数::生成する("draw_stage_resources/create/optional_stages.rs", "大気の上へ積む", "生成要求"),
    未是正の自由関数::生成する("draw_stage_resources/create/optional_stages.rs", "布シャドウを生成する", "生成要求"),
    未是正の自由関数::生成する("draw_stage_resources/create/optional_stages.rs", "組み立てる", "生成要求"),
    未是正の自由関数::生成する(
        "draw_stage_resources/create/optional_stages/distant_environment.rs",
        "生成する",
        "生成要求",
    ),
    未是正の自由関数::生成する(
        "draw_stage_resources/create/optional_stages/sky_stage.rs",
        "大気のベイク済み画像を生成する",
        "生成要求",
    ),
    未是正の自由関数::生成する("draw_stage_resources/create/optional_stages/sky_stage.rs", "空を生成する", "生成要求"),
    未是正の自由関数::生成する("scene_draw_resources/work_area/object_entry.rs", "全個体を束の段で積む", "描画対象の材料"),
    未是正の自由関数::生成する("scene_draw_resources/work_area/object_entry.rs", "全個体を束の段で積む", "描画発行受け皿"),
    未是正の自由関数::生成する(
        "scene_draw_resources/work_area/object_entry.rs",
        "動く個体の位置と向きを書き込む",
        "描画対象の材料",
    ),
    未是正の自由関数::生成する("scene_draw_resources/work_area/object_entry.rs", "積む", "描画対象の材料"),
    未是正の自由関数::生成する("scene_draw_resources/work_area/object_entry.rs", "積む", "描画発行受け皿"),
    未是正の自由関数::生成する("scene_draw_resources/work_area/stage_issue.rs", "積む", "描画発行受け皿"),
    未是正の自由関数::生成する("view_uniform_write.rs", "組み立てる", "ビュー射影の行列一式"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates/blitz_render/src/renderer", &項目一覧)
}
