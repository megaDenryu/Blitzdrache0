//! 描画1回ぶんのカメラ相対アンカーと、それを頂点ステージへ渡すプッシュ定数境界。
//! シーン・シャドウ・布・粒子の4本の頂点シェーダーが同じ16バイトを読むため、値の形とVulkan側の範囲宣言をここ1箇所が持つ。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「適用点: チャンク由来の描画アンカー」

use ash::vk;
use blitz_math::{カメラ相対位置, 大域ワールド位置, 座標変換エラー};

/// 16バイト境界へ揃えるため、xyzの3成分に余白1つを足した4成分で持つ。
const 成分数: usize = 4;
const バイト長: u32 = 16;

/// 頂点のワールド位置へ加算するカメラ相対のアンカー位置。
#[derive(Clone, Copy)]
pub(crate) struct カメラ相対アンカー {
    成分: [f32; 成分数],
}

impl カメラ相対アンカー {
    pub(crate) fn 相対位置から生成する(位置: カメラ相対位置) -> Self {
        Self {
            成分: [位置.x().値(), 位置.y().値(), 位置.z().値(), 0.0],
        }
    }

    /// 布と粒子の頂点位置は世界原点を基準にGPUで計算されるため、所有チャンクを持たない対象として世界原点をアンカーにする。
    pub(crate) fn 世界原点から生成する(カメラ大域原点: 大域ワールド位置) -> Result<Self, 座標変換エラー> {
        Ok(Self::相対位置から生成する(
            大域ワールド位置::原点().カメラ相対へ変換する(カメラ大域原点)?,
        ))
    }

    pub(crate) fn バイト列(self) -> [u8; 16] {
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
