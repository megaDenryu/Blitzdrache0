//! 呼び出し元が1つの単位として追加・解除する描画対象のGPU資源と、その描画対象だけを結ぶディスクリプタセットの束。
//! 束専用のディスクリプタプールを持つため、束の解除はプール1つの破棄で完結し、ディスクリプタセット添字はこの束の内側に閉じる。
//! 注意: 添字が束の内側で閉じることが、他の束の追加・解除で描画対象添字がずれないことの根拠である。

use ash::vk;

use super::create::チャンク描画資源生成材料;
use super::render_object_resources::{self, 描画対象資源};
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::terrain_detail_level::地形詳細段;
use crate::vulkan::descriptor::描画対象ディスクリプタプール;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct チャンク描画資源 {
    /// 呼び出し元が与えた識別子。解除するときはこの値で対象を特定する。
    id: 描画束ID,
    描画対象資源一覧: Vec<描画対象資源>,
    ディスクリプタ: 描画対象ディスクリプタプール,
}

impl チャンク描画資源 {
    /// 失敗したときは生成途中のGPU資源をすべて解放してからエラーを返すため、呼び出し元は自分が保持中の束をそのまま使い続けられる。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        材料: チャンク描画資源生成材料<'_>,
        id: 描画束ID,
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
            id,
            描画対象資源一覧,
            ディスクリプタ,
        })
    }

    pub(super) fn id(&self) -> 描画束ID {
        self.id
    }

    pub(super) fn 描画対象数(&self) -> usize {
        self.描画対象資源一覧.len()
    }

    /// 描画発行の作業領域を積むための走査。描画対象資源と、そのフレームで束縛すべきディスクリプタセットと、そのフレームで描く詳細段を組で返すため、呼び出し元は添字の対応規則を知らなくてよい。
    /// 詳細段は束ごとに決まるため、束の中の全描画対象へ同じ値を配る。
    pub(super) fn 描画対象と対応セット(
        &self,
        フレーム添字: フレームスロット添字,
        段: 地形詳細段,
    ) -> impl Iterator<Item = (&描画対象資源, vk::DescriptorSet, 地形詳細段)> {
        self.描画対象資源一覧
            .iter()
            .enumerate()
            .map(move |(添字, 資源)| (資源, self.ディスクリプタ.set(添字, フレーム添字), 段))
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
