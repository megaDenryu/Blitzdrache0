//! 選別のコンピュートのパス宣言。セルごとの区間表とクラスタ光添字列の2本へ書き込むことを宣言し、
//! 班の数だけコンピュートを積む。
//!
//! 書き込みの用途を宣言するのは、この2本を同じフレームの画素段が読むためである。宣言があることで、
//! コンピュートの書き込みから画素段の読みへのバリアをレンダーグラフが導く。スロットへ分けず1フレームの中の
//! バリアで守るのは、書いた内容をそのフレームの中でだけ使い、フレームをまたいで保たないためである
//! (参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断e」)。

use crate::vulkan::cluster_light_assignment::クラスタ選別の描画入力;
use crate::vulkan::graph::{
    GPU命令の積み先と宣言済み資源の取り出し口, バッファハンドル, バッファ用途, パス宣言, パス種別
};

pub(in crate::vulkan::frame) fn クラスタ選別パスを宣言する<'a>(
    格子: バッファハンドル,
    光添字列: バッファハンドル,
    入力: クラスタ選別の描画入力,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "クラスタの選別",
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![(格子, バッファ用途::コンピュート書き), (光添字列, バッファ用途::コンピュート書き)],
        パス種別::コンピュート,
        move |文脈| {
            let _ = 文脈.バッファを解決する(格子);
            let _ = 文脈.バッファを解決する(光添字列);
            クラスタ選別のgpu命令を積む(文脈, &入力);
        },
    )
}

fn クラスタ選別のgpu命令を積む(
    文脈: &GPU命令の積み先と宣言済み資源の取り出し口, 入力: &クラスタ選別の描画入力
) {
    let device = 文脈.積み先().論理デバイス();
    let command_buffer = 文脈.積み先().コマンドバッファ();
    let セット一覧 = [入力.セット];
    // 安全性: command_bufferは記録中で、pipeline・layout・セットは生成済み。即時定数の長さはレイアウトが宣言した範囲と一致する。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, ash::vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
        device.cmd_bind_descriptor_sets(command_buffer, ash::vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &セット一覧, &[]);
        device.cmd_push_constants(command_buffer, 入力.layout, ash::vk::ShaderStageFlags::COMPUTE, 0, &入力.即時定数);
        device.cmd_dispatch(command_buffer, 入力.班数, 1, 1);
    }
}
