//! シャドウ記録の1発行ぶんに頂点ステージへ送るプッシュ定数(カメラ相対アンカー + 帯番号)と、そのVulkan側の範囲宣言。
//! 通常のシャドウと布専用シャドウの2本の頂点シェーダーが同じ20バイトを読むため、値の形をここ1箇所が持つ。
//!
//! シーン・粒子が読む`relative_anchor`の16バイトと別に持つのは、帯番号を足した形をシャドウ以外のパイプラインへ
//! 波及させないためである。混ぜるとシーンのパイプラインレイアウトも20バイトを宣言することになり、
//! 帯を知らないシェーダーが帯の存在を宣言に持つことになる。

use ash::vk;

use crate::cascade::帯番号;
use crate::vulkan::relative_anchor::カメラ相対アンカー;

/// アンカーのfloat4(16バイト)に続けて帯番号のuint(4バイト)を置く。
const バイト長: u32 = 20;

/// パイプラインレイアウト生成時に宣言する範囲。頂点ステージだけが読む。
pub(crate) fn プッシュ定数範囲() -> vk::PushConstantRange {
    vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(バイト長)
}

/// 注意: 呼び出し元がコマンド記録中であることと、layoutがこの範囲を宣言済みであることを保証する。
pub(crate) unsafe fn 積む(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    アンカー: カメラ相対アンカー,
    番号: 帯番号,
) {
    let mut バイト列 = [0u8; 20];
    バイト列[..16].copy_from_slice(&アンカー.バイト列());
    バイト列[16..].copy_from_slice(&帯番号のgpu境界値(番号).to_le_bytes());
    // 安全性: 呼び出し元がコマンド記録中と、layoutが頂点ステージの20バイト範囲を宣言済みであることを保証する。
    unsafe {
        device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::VERTEX, 0, &バイト列);
    }
}

fn 帯番号のgpu境界値(番号: 帯番号) -> u32 {
    u32::try_from(番号.添字()).unwrap_or_else(|_| panic!("帯の添字がu32に収まらない: {}", 番号.添字()))
}
