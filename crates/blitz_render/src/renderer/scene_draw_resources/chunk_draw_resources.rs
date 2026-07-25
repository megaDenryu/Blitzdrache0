//! 所有チャンクが同じ描画対象のGPU資源と、その描画対象だけを結ぶディスクリプタセットの束。
//! チャンク専用のディスクリプタプールを持つため、チャンクの解除はプール1つの破棄で完結し、ディスクリプタセット添字はこのチャンクの内側に閉じる。
//! 注意: 添字がチャンクの内側で閉じることが、他のチャンクの追加・解除で描画対象添字がずれないことの根拠である。

use ash::vk;

use super::render_object_resources::{self, 描画対象資源};
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan::descriptor::{ディスクリプタレイアウト, 描画対象ディスクリプタプール};
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::uniform::フレームユニフォーム一式;

/// チャンク1つぶんの資源を作るためにチャンクの外から与える材料。ディスクリプタセットはフレームユニフォームとシャドウマップも結び、レイアウトはチャンクをまたいで共有するため、描画対象素材だけでは足りない。
pub(super) struct チャンク描画資源生成材料<'a> {
    pub(super) 物理デバイス問い合わせ: 物理デバイス問い合わせ<'a>,
    pub(super) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    pub(super) 転送環境: &'a 転送実行環境,
    pub(super) ユニフォーム: &'a フレームユニフォーム一式,
    pub(super) シャドウマップ: &'a シャドウマップ,
    pub(super) レイアウト: &'a ディスクリプタレイアウト,
}

pub(super) struct チャンク描画資源 {
    描画対象資源一覧: Vec<描画対象資源>,
    ディスクリプタ: 描画対象ディスクリプタプール,
}

impl チャンク描画資源 {
    /// 失敗したときは生成途中のGPU資源をすべて解放してからエラーを返すため、呼び出し元は自分が保持中のチャンクをそのまま使い続けられる。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        材料: チャンク描画資源生成材料<'_>,
        描画対象一覧: &[描画対象素材],
    ) -> Result<Self, レンダラーエラー> {
        let 描画対象資源一覧 = render_object_resources::描画対象資源一覧を生成する(
            材料.物理デバイス問い合わせ,
            device,
            材料.メモリプロパティ,
            材料.転送環境,
            描画対象一覧,
        )?;
        let ディスクリプタ = match ディスクリプタを生成する(device, 材料, &描画対象資源一覧) {
            Ok(値) => 値,
            Err(誤り) => {
                for 資源 in &描画対象資源一覧 {
                    資源.破棄する(device);
                }
                return Err(誤り);
            }
        };
        Ok(Self {
            描画対象資源一覧,
            ディスクリプタ,
        })
    }

    pub(super) fn 描画対象数(&self) -> usize {
        self.描画対象資源一覧.len()
    }

    /// 描画発行の作業領域を積むための走査。描画対象資源と、そのフレームで束縛すべきディスクリプタセットを対で返すため、呼び出し元は添字の対応規則を知らなくてよい。
    pub(super) fn 描画対象と対応セット(
        &self,
        フレーム添字: フレームスロット添字,
    ) -> impl Iterator<Item = (&描画対象資源, vk::DescriptorSet)> {
        self.描画対象資源一覧
            .iter()
            .enumerate()
            .map(move |(添字, 資源)| (資源, self.ディスクリプタ.set(添字, フレーム添字)))
    }

    /// 注意: ディスクリプタセットが指すテクスチャとユニフォームより先にディスクリプタプールを破棄する。
    /// 前提: 呼び出し元がGPU側の使用完了を待ってから呼ぶ。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ディスクリプタ.破棄する(device);
        for 資源 in &self.描画対象資源一覧 {
            資源.破棄する(device);
        }
    }
}

fn ディスクリプタを生成する(
    device: &GPUデバイス,
    材料: チャンク描画資源生成材料<'_>,
    描画対象資源一覧: &[描画対象資源],
) -> Result<描画対象ディスクリプタプール, レンダラーエラー> {
    let 参照一覧 = 描画対象資源一覧.iter().map(描画対象資源::ディスクリプタ参照).collect::<Vec<_>>();
    描画対象ディスクリプタプール::生成する(device, 材料.レイアウト, &参照一覧, 材料.ユニフォーム, 材料.シャドウマップ)
}
