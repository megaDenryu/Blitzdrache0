//! レンダラーの生成手順。インスタンス〜スワップチェーンは`core_setup`、残りの資源は`generate_resources`へ委ねる。

mod assemble;
mod core_setup;
mod debug_setup;
mod frame_resources;
mod generate_resources;
use super::レンダラー;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::particle_material::粒子素材;
use crate::render_scene_material::描画シーン素材;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

impl レンダラー {
    /// Vulkanインスタンスからスワップチェーン・各バッファ・各パイプライン・開発用UI(判断33)・ポストプロセス(判断38)・スキニング(判断44)・布(判断54)までを構築する。
    /// 前提: `表示ハンドル`と`ウィンドウハンドル`の指すウィンドウは、戻り値のレンダラーより長生きすること(呼び出し元のフィールド宣言順で担保する)。
    #[allow(clippy::too_many_arguments)]
    pub fn 生成する(
        表示ハンドル: RawDisplayHandle,
        ウィンドウハンドル: RawWindowHandle,
        寸法: ウィンドウ寸法,
        シェーダー: シェーダー束,
        描画シーン: 描画シーン素材,
        スキン: Option<スキンメッシュ素材>,
        布: Option<布素材>,
        粒子: Option<粒子素材>,
        ポスト処理有効: bool,
    ) -> Result<Self, レンダラーエラー> {
        let コア = core_setup::組み立てる(表示ハンドル, ウィンドウハンドル, 寸法)?;
        let 資源 = generate_resources::組み立てる(generate_resources::生成要求 {
            instance: &コア.instance,
            physical_device: コア.physical_device,
            device: &コア.device,
            queue: コア.queue,
            queue_family_index: コア.queue_family_index,
            swapchain: &コア.swapchain,
            シェーダー: &シェーダー,
            描画シーン: &描画シーン,
            スキン: スキン.as_ref(),
            布: 布.as_ref(),
            粒子素材: 粒子.as_ref(),
            ポスト処理有効,
            タイムスタンプ対応か: コア.タイムスタンプ対応か,
            タイムスタンプ周期ns: コア.タイムスタンプ周期ns,
        })?;
        Ok(assemble::レンダラーを組み立てる(コア, 資源, 寸法))
    }
}
