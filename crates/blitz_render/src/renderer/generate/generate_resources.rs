//! スワップチェーン生成後に組み立てる残り資源の組み立て手順。段ごとの生成はサブモジュールへ分ける(基礎/フレーム送信/パイプライン/粒子/ポスト処理/シミュレーション)。束の型は`frame_resources`。
//! GPU計測と開発用UIはそれぞれ1つの生成呼び出しで済み、この手順が足す判断も条件も無いため、サブモジュールを設けずここに置く。

mod base_resources;
mod bundle;
mod cloth_resources;
mod particle_resources;
mod pipeline_resources;
mod post_process_resources;
mod request;
mod simulation_resources;
mod submission_resources;

use super::frame_resources::フレーム資源;
use crate::error::レンダラーエラー;
use crate::vulkan;
use post_process_resources::描画先構成;

pub(in crate::renderer::generate) use request::生成要求;

pub(super) fn 組み立てる(要求: 生成要求<'_>) -> Result<フレーム資源, レンダラーエラー> {
    let 描画先 = 描画先構成::決める(要求.フレーム構成);
    let シーンカラー形式 = 描画先.シーンカラー形式(要求.swapchain.画像形式);

    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    let メモリプロパティ = unsafe { 要求.instance.get_physical_device_memory_properties(要求.physical_device) };
    let 基礎 = base_resources::組み立てる(
        要求.instance,
        要求.physical_device,
        要求.device,
        要求.queue,
        要求.queue_family_index,
        &メモリプロパティ,
        要求.swapchain.寸法,
        要求.描画シーン,
    )?;

    let 送信 = submission_resources::組み立てる(要求.device, 要求.queue_family_index, 要求.swapchain)?;
    let パイプライン =
        pipeline_resources::組み立てる(要求.device, シーンカラー形式, 基礎.シーン描画資源.ディスクリプタlayout(), 要求.シェーダー)?;
    let 粒子 = particle_resources::組み立てる(&要求, &メモリプロパティ, &基礎, シーンカラー形式)?;
    let gpu計測 = vulkan::gpu_timing::パス別GPU計測::生成する(要求.device, 要求.タイムスタンプ対応か, 要求.タイムスタンプ周期ns)?;
    let ui一式 = vulkan::ui::UIリソース一式::生成する(要求.device, 要求.swapchain.画像形式, &要求.シェーダー.ui)?;
    let ポスト処理 = 描画先.組み立てる(要求.device, &メモリプロパティ, 要求.swapchain, 要求.シェーダー)?;
    let (スキニング, 布) = simulation_resources::組み立てる(
        要求.device,
        &メモリプロパティ,
        &基礎.転送環境,
        シーンカラー形式,
        基礎.シーン描画資源.ディスクリプタlayout(),
        要求.描画シーン.先頭の描画対象().頂点一覧(),
        要求.スキン,
        要求.布,
        要求.シェーダー,
    )?;

    Ok(bundle::束ねる(bundle::段別資源 {
        基礎,
        送信,
        パイプライン,
        粒子,
        gpu計測,
        ui一式,
        ポスト処理,
        スキニング,
        布,
    }))
}
