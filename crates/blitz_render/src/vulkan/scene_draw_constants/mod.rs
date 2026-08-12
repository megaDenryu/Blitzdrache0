//! シーン描画1回ぶんの描画定数と、それを頂点ステージ・画素段ステージへ渡すプッシュ定数境界。
//! 運ぶのはカメラ相対の基準原点と、その発行が塗る材質のレコード添字である。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」
//!
//! 基準原点の16バイトのうち余白だった4バイトへ材質レコード添字を置くため、プッシュ定数の総量は増えない。
//! 布・粒子が読む`relative_anchor`の16バイトと別に持つのは、材質を持たないパイプラインへ材質レコード添字を
//! 波及させないためである。シャドウの20バイト定数(`shadow_push`)とも別のままにする。

#[cfg(test)]
mod layout_tests;

use ash::vk;

use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

const バイト長: u32 = 16;
/// 基準原点のxyzが占める12バイトの直後。旧の余白4バイトがここである。
const 材質レコード添字の開始位置: usize = 12;

#[derive(Clone, Copy)]
pub(crate) struct シーン描画定数 {
    基準原点: カメラ相対の基準原点,
    /// その描画対象が持つ材質レコード列の中での位置。世代に拘束されない束の内側の添字である。
    材質レコード添字: u32,
}

impl シーン描画定数 {
    pub(crate) fn 生成する(基準原点: カメラ相対の基準原点, 材質レコード添字: u32) -> Self {
        Self {
            基準原点, 材質レコード添字
        }
    }

    fn バイト列(self) -> [u8; 16] {
        let mut バイト列 = self.基準原点.バイト列();
        バイト列[材質レコード添字の開始位置..].copy_from_slice(&self.材質レコード添字.to_le_bytes());
        バイト列
    }
}

/// パイプラインレイアウト生成時に宣言する範囲。頂点ステージが基準原点を、画素段ステージが材質レコード添字を読む。
pub(crate) fn プッシュ定数範囲() -> vk::PushConstantRange {
    vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
        .offset(0)
        .size(バイト長)
}

/// 注意: 呼び出し元がコマンド記録中であることと、layoutがこの範囲を宣言済みであることを保証する。
pub(crate) unsafe fn 積む(積み先: GPU命令の積み先<'_>, layout: vk::PipelineLayout, 描画定数: シーン描画定数) {
    let device = 積み先.論理デバイス();
    let command_buffer = 積み先.コマンドバッファ();
    // 安全性: 呼び出し元がコマンド記録中と、layoutが両ステージの16バイト範囲を宣言済みであることを保証する。
    unsafe {
        device.cmd_push_constants(
            command_buffer,
            layout,
            vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            0,
            &描画定数.バイト列(),
        );
    }
}
