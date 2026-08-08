//! 局所可視性補正の2本のパスの宣言。遮蔽の標本化が深度から生の可視度を書き、両側ぼかしが同じ面に載る画素の間だけで均す。
//! 2本を1つのファイルへ置くのは、この順そのものが担当する関心事だからである。ぼかしを先に置けば前フレームの生の可視度を
//! 均すことになり、標本の回転の違いが1フレーム遅れて現れる。
//! 注意: どちらのパスも深度を`深度コンピュート読み`で読む。深度プリパスが書いた後・シーン描画が読む前に置くことは
//! 積み込む側(`graph_build`)の担当であり、この2本は順序を1つも知らない。

use ash::vk;

use crate::vulkan::graph::{
    GPU命令の積み先と宣言済み資源の取り出し口, パス宣言, パス種別, 画像ハンドル, 画像用途
};
use crate::vulkan::local_visibility::局所可視性描画入力;

/// 1班が覆う画素の一辺。`shaders/local_visibility_occlusion.slang`と`local_visibility_blur.slang`の
/// `threadsPerSide`と一致させる。食い違うと画面の一部が書かれないまま残る。
const 班の一辺: u32 = 8;

pub(super) fn 遮蔽の標本化を作る<'a>(
    深度: 画像ハンドル,
    生: 画像ハンドル,
    入力: &'a 局所可視性描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "局所可視性の遮蔽の標本化",
        vec![(深度, 画像用途::深度コンピュート読み)],
        vec![(生, 画像用途::コンピュート書き)],
        Vec::new(),
        Vec::new(),
        パス種別::コンピュート,
        move |文脈| コンピュートを積む(文脈, 入力, 入力.遮蔽の標本化pipeline, 班数(寸法)),
    )
}

pub(super) fn 両側ぼかしを作る<'a>(
    深度: 画像ハンドル,
    生: 画像ハンドル,
    ぼかし後: 画像ハンドル,
    入力: &'a 局所可視性描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "局所可視性の両側ぼかし",
        vec![(深度, 画像用途::深度コンピュート読み), (生, 画像用途::コンピュート記憶読み)],
        vec![(ぼかし後, 画像用途::コンピュート書き)],
        Vec::new(),
        Vec::new(),
        パス種別::コンピュート,
        move |文脈| コンピュートを積む(文脈, 入力, 入力.両側ぼかしpipeline, 班数(寸法)),
    )
}

/// 画面を覆うのに要る班の数。端の班がはみ出す画素は、シェーダーの側が寸法との比較で捨てる。
fn 班数(寸法: vk::Extent2D) -> [u32; 3] {
    [寸法.width.div_ceil(班の一辺), 寸法.height.div_ceil(班の一辺), 1]
}

fn コンピュートを積む(
    文脈: &GPU命令の積み先と宣言済み資源の取り出し口, 入力: &局所可視性描画入力, pipeline: vk::Pipeline, 班数: [u32; 3]
) {
    let device = 文脈.device();
    let command_buffer = 文脈.コマンドバッファ();
    let セット一覧 = [入力.セット];
    // 安全性: command_bufferは記録中で、pipeline・layout・セットは生成済み。定数の長さはレイアウトが宣言した範囲と一致する。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
        device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &セット一覧, &[]);
        device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::COMPUTE, 0, &入力.即時定数);
        device.cmd_dispatch(command_buffer, 班数[0], 班数[1], 班数[2]);
    }
}
