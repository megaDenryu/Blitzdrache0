//! シャドウ記録の1発行ぶんの描画定数(カメラ相対の基準原点 + 距離区分番号)と、そのVulkan側の範囲宣言。
//! 通常のシャドウと布専用シャドウの2本の頂点シェーダーが同じ20バイトを読むため、値の形をここ1箇所が持つ。
//!
//! シーン・粒子が読む`relative_anchor`の16バイトと別に持つのは、距離区分番号を足した形をシャドウ以外のパイプラインへ
//! 波及させないためである。混ぜるとシーンのパイプラインレイアウトも20バイトを宣言することになり、
//! 距離区分を知らないシェーダーが距離区分の存在を宣言に持つことになる。

use ash::vk;

use crate::cascade::距離区分番号;
use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

/// 基準原点のfloat4(16バイト)に続けて距離区分番号のuint(4バイト)を置く。
const バイト長: u32 = 20;

#[derive(Clone, Copy)]
pub(crate) struct シャドウ描画定数 {
    基準原点: カメラ相対の基準原点,
    距離区分: 距離区分番号, // その発行が書き込む距離区分。シェーダーは層の添字として読む
}

impl シャドウ描画定数 {
    pub(crate) fn 生成する(基準原点: カメラ相対の基準原点, 距離区分: 距離区分番号) -> Self {
        Self { 基準原点, 距離区分 }
    }

    fn バイト列(self) -> [u8; 20] {
        let mut バイト列 = [0u8; 20];
        バイト列[..16].copy_from_slice(&self.基準原点.バイト列());
        バイト列[16..].copy_from_slice(&self.距離区分のgpu境界値().to_le_bytes());
        バイト列
    }

    fn 距離区分のgpu境界値(self) -> u32 {
        u32::try_from(self.距離区分.添字()).unwrap_or_else(|_| panic!("距離区分の添字がu32に収まらない: {}", self.距離区分.添字()))
    }

    /// パイプラインレイアウト生成時に宣言する範囲。頂点ステージだけが読む。
    pub(crate) fn プッシュ定数範囲() -> vk::PushConstantRange {
        vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(バイト長)
    }

    /// 注意: 呼び出し元がコマンド記録中であることと、layoutがこの範囲を宣言済みであることを保証する。
    pub(crate) unsafe fn プッシュ定数として積む(self, 積み先: GPU命令の積み先<'_>, layout: vk::PipelineLayout) {
        // 安全性: 呼び出し元がコマンド記録中と、layoutが頂点ステージの20バイト範囲を宣言済みであることを保証する。
        unsafe {
            積み先
                .論理デバイス()
                .cmd_push_constants(積み先.コマンドバッファ(), layout, vk::ShaderStageFlags::VERTEX, 0, &self.バイト列());
        }
    }
}
