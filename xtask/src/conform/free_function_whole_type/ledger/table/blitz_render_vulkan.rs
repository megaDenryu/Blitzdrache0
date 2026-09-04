//! `blitz_render`のVulkan層の未是正の自由関数の一覧。ディスクリプタの束縛先と、焼く条件をまとめた型を丸ごと受け取るものが並ぶ。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 37] = [
    未是正の自由関数::生成する(
        "atmosphere_lut/composite_descriptor/binding.rs",
        "ボリュームを書き込む",
        "空中遠近合成の束縛先",
    ),
    未是正の自由関数::生成する(
        "atmosphere_lut/composite_descriptor/create.rs",
        "空中遠近合成ディスクリプタを生成する",
        "空中遠近合成の束縛先",
    ),
    未是正の自由関数::生成する("atmosphere_lut/draw_input/pass_binding.rs", "空中遠近の押し込みを作る", "描画入力の材料"),
    未是正の自由関数::生成する("atmosphere_lut/march_descriptor/binding.rs", "書き込む", "経路生成の束縛先"),
    未是正の自由関数::生成する("atmosphere_lut/multiscatter_descriptor/binding.rs", "書き込む", "多重散乱の束縛先"),
    未是正の自由関数::生成する("atmosphere_lut/probe.rs", "一式で焼く", "大気のベイク済み画像一式"),
    未是正の自由関数::生成する("atmosphere_lut/probe.rs", "一式で焼く", "焼く条件"),
    未是正の自由関数::生成する("atmosphere_lut/probe.rs", "環境で焼く", "焼く条件"),
    未是正の自由関数::生成する(
        "atmosphere_lut/sample_descriptor/binding.rs",
        "書き込む",
        "大気のベイク済み画像標本の束縛先",
    ),
    未是正の自由関数::生成する(
        "atmosphere_lut/sample_descriptor/create.rs",
        "大気のベイク済み画像標本ディスクリプタを生成する",
        "大気のベイク済み画像標本の束縛先",
    ),
    未是正の自由関数::生成する("cloth/params.rs", "バイト列にする", "固定部"),
    未是正の自由関数::生成する("derived_environment/probe/plan.rs", "テクセル数を求める", "派生表現を焼く条件"),
    未是正の自由関数::生成する("derived_environment/probe/plan.rs", "鏡面畳込みの入力一覧", "派生表現を焼く条件"),
    未是正の自由関数::生成する("derived_environment/probe/plan.rs", "鏡面畳込みの入力一覧", "派生表現一式"),
    未是正の自由関数::生成する("derived_environment/probe/record.rs", "一式の上で焼く", "派生表現を焼く条件"),
    未是正の自由関数::生成する("derived_environment/probe/record.rs", "一式の上で焼く", "派生表現一式"),
    未是正の自由関数::生成する("derived_environment/probe/record.rs", "環境で焼く", "派生表現を焼く条件"),
    未是正の自由関数::生成する("derived_environment/probe/record.rs", "遠方環境の上で焼く", "派生表現を焼く条件"),
    未是正の自由関数::生成する(
        "distant_environment/descriptor/binding.rs",
        "遠方環境の束縛先をディスクリプタセットへ書き込む",
        "遠方環境の束縛先",
    ),
    未是正の自由関数::生成する("distant_environment/probe/graph_build.rs", "大気の三枚を積む", "積む材料"),
    未是正の自由関数::生成する("distant_environment/probe/record.rs", "受け皿を用意して焼く", "遠方環境を焼く条件"),
    未是正の自由関数::生成する("distant_environment/probe/record.rs", "受け皿を用意して焼く", "遠方環境一式"),
    未是正の自由関数::生成する("distant_environment/probe/record.rs", "大気の上で焼く", "遠方環境を焼く条件"),
    未是正の自由関数::生成する("distant_environment/probe/record.rs", "環境で焼く", "遠方環境を焼く条件"),
    未是正の自由関数::生成する("frame/record/cloth_passes/collision_stage.rs", "空間ハッシュと分離を積む", "布ハンドル"),
    未是正の自由関数::生成する("frame/record/cloth_passes/constraint_stage.rs", "拘束を積む", "布ハンドル"),
    未是正の自由関数::生成する(
        "frame/record/cloth_passes/constraint_stage.rs",
        "目標拘束の最終の成立を積む",
        "布ハンドル",
    ),
    未是正の自由関数::生成する("frame/record/cloth_passes/constraint_stage.rs", "目標拘束を一回積む", "布ハンドル"),
    未是正の自由関数::生成する("frame/record/cloth_passes/sequence.rs", "一刻みを積む", "布ハンドル"),
    未是正の自由関数::生成する("frame/record/cloth_passes/sequence.rs", "積む", "布ハンドル"),
    未是正の自由関数::生成する("frame/record/scene_pass.rs", "布を記録する", "布ドロー"),
    未是正の自由関数::生成する("headless/session.rs", "一時コマンドバッファを1本確保する", "ウィンドウなし実行GPU環境"),
    未是正の自由関数::生成する(
        "pipeline_ledger/device_supplier/family_dispatch.rs",
        "生成する",
        "デバイスパイプライン供給元",
    ),
    未是正の自由関数::生成する("post_process/create.rs", "束を生成する", "生成材料"),
    未是正の自由関数::生成する("sky_stage/create.rs", "パイプラインを作る", "空段階の生成要求"),
    未是正の自由関数::生成する("sky_stage/create.rs", "合成を作る", "空段階の生成要求"),
    未是正の自由関数::生成する("transfer/session.rs", "一時コマンドバッファを1本確保する", "転送実行環境"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates/blitz_render/src/vulkan", &項目一覧)
}
