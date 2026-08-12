//! 点光源の影の記録の1発行ぶんの描画定数(カメラ相対の基準原点 + その面のライトビュー射影)と、
//! そのVulkan側の範囲宣言。値の形はSlangの`point_light_shadow_push.slang`と同じ80バイトである。
//!
//! 面の行列をプッシュ定数で運ぶのは、面ごとの行列が最大24本あり、そのうち1本だけをその発行が読むためである。
//! 24本を定数バッファへ載せて添字で選ぶ形にすると、影を1件も持たない世界も24本ぶんの領域を持つことになる。

use ash::vk;

use blitz_math::{ワールド, 変換, 点光源の面クリップ};

use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

/// 基準原点のfloat4(16バイト)に続けて列優先の4x4行列(64バイト)を置く。
const バイト長: u32 = 80;

#[derive(Clone, Copy)]
pub(crate) struct 点光源の影の描画定数 {
    /// 前提: 基準原点と面のライトビュー射影は同じカメラ大域原点で狭めた値である。原点が違うと影が世界ごとずれる。
    基準原点: カメラ相対の基準原点,
    面のライトビュー射影: 変換<ワールド, 点光源の面クリップ>,
}

impl 点光源の影の描画定数 {
    pub(crate) fn 生成する(
        基準原点: カメラ相対の基準原点, 面のライトビュー射影: 変換<ワールド, 点光源の面クリップ>
    ) -> Self {
        Self {
            基準原点,
            面のライトビュー射影,
        }
    }

    fn バイト列(self) -> [u8; 80] {
        let mut バイト列 = [0u8; 80];
        バイト列[..16].copy_from_slice(&self.基準原点.バイト列());
        let 列優先 = self.面のライトビュー射影.gpu境界用列優先配列();
        for (列添字, 列) in 列優先.iter().enumerate() {
            for (行添字, 成分) in 列.iter().enumerate() {
                let 先頭 = 16 + (列添字 * 4 + 行添字) * 4;
                バイト列[先頭..先頭 + 4].copy_from_slice(&成分.to_le_bytes());
            }
        }
        バイト列
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
        // 安全性: 呼び出し元がコマンド記録中と、layoutが頂点ステージの80バイト範囲を宣言済みであることを保証する。
        unsafe {
            積み先
                .論理デバイス()
                .cmd_push_constants(積み先.コマンドバッファ(), layout, vk::ShaderStageFlags::VERTEX, 0, &self.バイト列());
        }
    }
}
