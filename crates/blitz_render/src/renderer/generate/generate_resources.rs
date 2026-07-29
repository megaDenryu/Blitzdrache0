//! 提示側の資源の生成後に組み立てる残り資源の組み立て手順。段ごとの生成はサブモジュールへ分ける(基礎/パイプライン/粒子/ポスト処理/シミュレーション)。束の型は`frame_resources`。
//! GPU計測・開発用UI・フレーム進行はそれぞれ1つの生成呼び出しで済み、この手順が足す判断も条件も無いため、サブモジュールを設けずここに置く。

mod base_resources;
mod bundle;
mod cloth_resources;
mod particle_resources;
mod post_process_resources;
mod request;
mod simulation_resources;

use super::frame_resources::フレーム資源;
use crate::error::レンダラーエラー;
use crate::renderer::draw_stage_resources::描画段階資源;
use crate::renderer::frame_progress::フレーム進行;
use crate::vulkan;
use post_process_resources::描画先構成;

pub(in crate::renderer::generate) use request::生成要求;

pub(super) fn 組み立てる(要求: 生成要求<'_>) -> Result<フレーム資源, レンダラーエラー> {
    let 描画先 = 描画先構成::決める(要求.フレーム構成);
    let シーンカラー形式 = 描画先.シーンカラー形式(要求.提示.画像形式());
    let device = 要求.環境.device();

    let メモリプロパティ = 要求.環境.メモリプロパティを取得する();
    let 基礎 = base_resources::組み立てる(要求.環境, &メモリプロパティ, 要求.描画シーン)?;

    let フレーム進行 = フレーム進行::生成する(device, 要求.環境.キューファミリ添字())?;
    let 描画段階 = 描画段階資源::生成する(device, シーンカラー形式, 基礎.シーン描画資源.ディスクリプタlayout(), 要求.シェーダー)?;
    let 粒子 = particle_resources::組み立てる(&要求, &メモリプロパティ, &基礎, シーンカラー形式)?;
    let gpu計測 = vulkan::gpu_timing::パス別GPU計測::生成する(device, 要求.タイムスタンプ対応か, 要求.タイムスタンプ周期ns)?;
    let ui一式 = vulkan::ui::UIリソース一式::生成する(device, 要求.提示.画像形式(), &要求.シェーダー.ui)?;
    let ポスト処理 = 描画先.組み立てる(device, &メモリプロパティ, 要求.提示, 要求.シェーダー)?;
    let (スキニング, 布) = simulation_resources::組み立てる(
        device,
        &メモリプロパティ,
        &基礎.転送環境,
        シーンカラー形式,
        基礎.シーン描画資源.ディスクリプタlayout(),
        要求.描画シーン.先頭の描画対象().最詳細段の頂点一覧(),
        要求.スキン,
        要求.布,
        要求.シェーダー,
    )?;

    Ok(bundle::束ねる(bundle::段別資源 {
        基礎,
        フレーム進行,
        描画段階,
        粒子,
        gpu計測,
        ui一式,
        ポスト処理,
        スキニング,
        布,
    }))
}
