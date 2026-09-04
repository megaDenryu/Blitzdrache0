//! `xtask`の未是正の自由関数の一覧。検収の条件と実測をまとめた型を丸ごと受け取るものが並ぶ。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 54] = [
    未是正の自由関数::生成する("auto_exposure/run.rs", "描画して報告を読む", "探り色の扱い"),
    未是正の自由関数::生成する("auto_exposure/run.rs", "起動指定を組み立てる", "探り色の扱い"),
    未是正の自由関数::生成する("cloth_night/judgment.rs", "明るさの時刻への追従を判定する", "布領域"),
    未是正の自由関数::生成する("conform/module_import_boundary.rs", "境界を1つ確かめる", "取り込み"),
    未是正の自由関数::生成する("csm_seam/summary.rs", "検収の要約を組み立てる", "検収の実測"),
    未是正の自由関数::生成する("csm_seam/summary.rs", "段差の記述", "検収の実測"),
    未是正の自由関数::生成する("depth_prepass_cost/record/write.rs", "行にする", "一標本"),
    未是正の自由関数::生成する("depth_prepass_cost/run.rs", "一回走らせる", "実行の材料"),
    未是正の自由関数::生成する("depth_prepass_cost/run.rs", "起動指定を組み立てる", "実行の材料"),
    未是正の自由関数::生成する("ibl_step/control.rs", "一枚を読む", "対照の対象"),
    未是正の自由関数::生成する("ibl_step/control.rs", "判定する", "対照の対象"),
    未是正の自由関数::生成する("ibl_step/control.rs", "後始末する", "対照の対象"),
    未是正の自由関数::生成する("ibl_step/control.rs", "撮影1枚の実行名", "対照の対象"),
    未是正の自由関数::生成する("ibl_step/measure.rs", "一枚を読む", "測る材料"),
    未是正の自由関数::生成する("ibl_step/measure.rs", "跨ぎを測る", "測る材料"),
    未是正の自由関数::生成する("ibl_step/record.rs", "行にする", "生値の行"),
    未是正の自由関数::生成する("ibl_step/run.rs", "起動指定を組み立てる", "撮るもの"),
    未是正の自由関数::生成する("ibl_step/scan.rs", "一件を測る", "走査の結果"),
    未是正の自由関数::生成する("indirect_cost/record.rs", "行にする", "一標本"),
    未是正の自由関数::生成する("indirect_cost/run.rs", "一回走らせる", "実行の材料"),
    未是正の自由関数::生成する("indirect_cost/run.rs", "起動指定を組み立てる", "実行の材料"),
    未是正の自由関数::生成する("indirect_probe/run.rs", "起動指定を組み立てる", "注入の指定"),
    未是正の自由関数::生成する("indirect_probe/summary.rs", "要約を組む", "要約の材料"),
    未是正の自由関数::生成する("instance_cull/judgment.rs", "正の判定を検査する", "実行の対"),
    未是正の自由関数::生成する("instance_cull/judgment.rs", "負の対照と影を検査する", "実行の対"),
    未是正の自由関数::生成する("instance_cull/judgment/common.rs", "両方の実行に共通の条件を検査する", "実行の対"),
    未是正の自由関数::生成する("instance_lod/judgment/tier.rs", "段が同時に立つことを検査する", "実行"),
    未是正の自由関数::生成する("instance_lod/judgment/tier.rs", "段が振動しないことを検査する", "実行"),
    未是正の自由関数::生成する("instance_lod/judgment/tier.rs", "段の違いが絵に出ることを検査する", "実行"),
    未是正の自由関数::生成する("memory_sampling.rs", "実行しながら採取する", "採取条件"),
    未是正の自由関数::生成する("memory_sampling.rs", "起動する", "採取条件"),
    未是正の自由関数::生成する("origin_invariance/run.rs", "起動指定を組み立てる", "検査条件"),
    未是正の自由関数::生成する("ow4_bench/condition.rs", "時刻の起動指定", "計測条件"),
    未是正の自由関数::生成する("pixel_region.rs", "全画素が範囲内かを確かめる", "画面領域"),
    未是正の自由関数::生成する("pixel_region.rs", "明度が範囲内の画素を数える", "画面領域"),
    未是正の自由関数::生成する("point_light_shadow/region.rs", "矩形の平均輝度を採る", "画面の矩形"),
    未是正の自由関数::生成する("point_light_shadow/region.rs", "辺の画素範囲を求める", "画面の矩形"),
    未是正の自由関数::生成する("report_parse.rs", "全パスを並べる", "計数報告"),
    未是正の自由関数::生成する("shadow_loss/compare.rs", "積む", "帯の集計"),
    未是正の自由関数::生成する("shadow_probe/record.rs", "行にする", "一標本"),
    未是正の自由関数::生成する("shadow_probe/run.rs", "一回走らせる", "実行の材料"),
    未是正の自由関数::生成する("shadow_probe/run.rs", "一日内時刻の秒", "実行の材料"),
    未是正の自由関数::生成する("shadow_probe/run.rs", "引数を作る", "実行の材料"),
    未是正の自由関数::生成する("sky_time/run.rs", "条件別の選択肢を足す", "条件"),
    未是正の自由関数::生成する("sky_time/run.rs", "起動指定を組み立てる", "条件"),
    未是正の自由関数::生成する("smoke/launch_setting.rs", "基本の指定", "起動設定"),
    未是正の自由関数::生成する("streaming_bench/order_compare.rs", "一件を表示する", "順序比較"),
    未是正の自由関数::生成する("temporal_reconstruction/injection.rs", "要約を組む", "注入の観測"),
    未是正の自由関数::生成する("temporal_reconstruction/injection/judgment.rs", "判定する", "注入の観測"),
    未是正の自由関数::生成する("temporal_reconstruction/injection/judgment.rs", "相対差の判定名", "注入の観測"),
    未是正の自由関数::生成する("temporal_reconstruction/observation.rs", "要約を組む", "観測"),
    未是正の自由関数::生成する("terrain_visual/band.rs", "破綻防止帯を判定する", "地面を照らす光"),
    未是正の自由関数::生成する("texture_compression/difference.rs", "画素1つを統計へ加える", "画素の成分差の統計"),
    未是正の自由関数::生成する("type_metrics/metrics.rs", "取り込む", "型計測"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("xtask/src", &項目一覧)
}
