//! レンダラーの生成手順。インスタンス〜スワップチェーンは`core_setup`、残りの資源は`generate_resources`へ委ねる。

mod core_setup;
mod debug_setup;
mod frame_resources;
mod generate_resources;
use super::レンダラー;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::material::マテリアル素材;
use crate::particle_material::粒子素材;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
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
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
        マテリアル: マテリアル素材,
        スキン: Option<スキンメッシュ素材>,
        布: Option<布素材>,
        粒子: Option<粒子素材>,
        ポスト処理有効: bool,
    ) -> Result<Self, レンダラーエラー> {
        let コア = core_setup::組み立てる(表示ハンドル, ウィンドウハンドル, 寸法)?;
        let 資源 = generate_resources::組み立てる(
            &コア.instance,
            コア.physical_device,
            &コア.device,
            コア.queue,
            コア.queue_family_index,
            &コア.swapchain,
            &シェーダー,
            頂点一覧,
            インデックス一覧,
            &マテリアル,
            スキン.as_ref(),
            布.as_ref(),
            粒子.as_ref(),
            ポスト処理有効,
            コア.タイムスタンプ対応か,
            コア.タイムスタンプ周期ns,
        )?;

        Ok(Self {
            entry: コア.entry,
            instance: コア.instance,
            デバッグメッセンジャー: コア.デバッグメッセンジャー,
            surface_loader: コア.surface_loader,
            surface: コア.surface,
            physical_device: コア.physical_device,
            device: コア.device,
            queue: コア.queue,
            swapchain_loader: コア.swapchain_loader,
            swapchain: コア.swapchain,
            深度バッファ: 資源.深度バッファ,
            シャドウマップ: 資源.シャドウマップ,
            シャドウパイプライン: 資源.シャドウパイプライン,
            転送環境: 資源.転送環境,
            ジオメトリ: 資源.ジオメトリ,
            テクスチャ: 資源.テクスチャ,
            ユニフォーム: 資源.ユニフォーム,
            ディスクリプタ: 資源.ディスクリプタ,
            command_pool: 資源.command_pool,
            command_buffer一覧: 資源.command_buffer一覧,
            フレーム同期: 資源.フレーム同期,
            提示同期: 資源.提示同期,
            現在フレーム添字: 0,
            pipeline: 資源.pipeline,
            スキニング: 資源.スキニング,
            布: 資源.布,
            hdrターゲット: 資源.hdrターゲット,
            ブルームピラミッド: 資源.ブルームピラミッド,
            ブルーム: 資源.ブルーム,
            トーンマップ: 資源.トーンマップ,
            粒子: 資源.粒子,
            gpu計測: 資源.gpu計測,
            cpu区間計測: None,
            ui一式: 資源.ui一式,
            読み戻しバッファ: None,
            検証カウンタ: コア.検証カウンタ,
            現在の寸法: 寸法,
            再構築が必要: false,
            ベースカラー係数: マテリアル.ベースカラー係数(),
            金属度係数: マテリアル.金属度係数(),
            粗さ係数: マテリアル.粗さ係数(),
        })
    }
}
