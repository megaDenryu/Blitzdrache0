//! 描画1回ぶんのカメラ相対アンカーと、それを頂点ステージへ渡すプッシュ定数境界。
//! シーン・シャドウ・布・粒子の4本の頂点シェーダーが同じ16バイトを読むため、値の形とVulkan側の範囲宣言をここ1箇所が持つ。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「適用点: チャンク由来の描画アンカー」

use ash::vk;

/// 16バイト境界へ揃えるため、xyzの3成分に余白1つを足した4成分で持つ。
const 成分数: usize = 4;
const バイト長: u32 = 16;

/// 頂点のワールド位置へ加算するカメラ相対のアンカー位置。
#[derive(Clone, Copy)]
pub(crate) struct カメラ相対アンカー {
    成分: [f32; 成分数],
}

impl カメラ相対アンカー {
    /// カメラ大域原点そのものへ乗るアンカー。頂点位置へ何も足さない。
    pub(crate) fn 加算なし() -> Self {
        Self { 成分: [0.0; 成分数] }
    }

    fn バイト列(self) -> [u8; 16] {
        let mut バイト列 = [0u8; 16];
        for (添字, 成分) in self.成分.iter().enumerate() {
            let 先頭 = 添字 * 4;
            バイト列[先頭..先頭 + 4].copy_from_slice(&成分.to_le_bytes());
        }
        バイト列
    }
}

/// パイプラインレイアウト生成時に宣言する範囲。頂点ステージだけが読む。
pub(crate) fn プッシュ定数範囲() -> vk::PushConstantRange {
    vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(バイト長)
}

/// 注意: 呼び出し元がコマンド記録中であることと、layoutがこの範囲を宣言済みであることを保証する。
pub(crate) unsafe fn 積む(
    device: &ash::Device, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, アンカー: カメラ相対アンカー
) {
    // 安全性: 呼び出し元がコマンド記録中と、layoutが頂点ステージの16バイト範囲を宣言済みであることを保証する。
    unsafe {
        device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::VERTEX, 0, &アンカー.バイト列());
    }
}
