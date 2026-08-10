//! 描画シーン素材から束を一括生成する。描画シーン素材の全描画対象を1つの束としてまとめ、その描画対象数に合わせて作業領域の容量を決める。
//! 途中で失敗したときに生成済みのGPU資源を解放する経路をここ1箇所へ閉じるため、生成時と差し替え時で同じ解放を二重に書かない。
//! 束の生成材料をここが定義するのは、材料の中身が「生成に何が要るか」であり、束そのものの構造ではないためである。

use ash::vk;

use super::chunk_draw_resources::チャンク描画資源;
use super::シーン描画資源;
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::render_scene_material::{動く個体の宣言, 描画シーン素材};
use crate::vulkan::descriptor::シーンセットレイアウト一式;
use crate::vulkan::material_table::描画対象別の材質ID;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

/// 起動時のシーン全体を1つの束として持つときの識別子。ストリーミングを使わない経路ではこの束だけが存在する。
pub(in crate::renderer) const 起動シーンの束ID: 描画束ID = 描画束ID::生成する(0);

/// 束1つぶんの資源を作るために束の外から与える材料。セットレイアウトと材質IDは束をまたぐ所有者から来るため、
/// 描画対象素材だけでは足りない。束を1つ追加するときに与える材料も同じ中身であるため、`束追加材料`はこの型の別名である。
pub(in crate::renderer) struct チャンク描画資源生成材料<'a> {
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    pub(in crate::renderer) 転送環境: &'a 転送実行環境,
    pub(in crate::renderer) セットレイアウト: &'a シーンセットレイアウト一式,
    /// 材質資源表がこの束の描画対象一覧と同じ並びで発番した大域材質ID。
    pub(in crate::renderer) 材質id一覧: &'a [描画対象別の材質ID],
    /// この束の中で毎フレーム変換を書き換える個体の宣言。チャンクの束は1件も持たない。
    pub(in crate::renderer) 動く個体一覧: &'a [動く個体の宣言],
}

pub(in crate::renderer) type 束追加材料<'a> = チャンク描画資源生成材料<'a>;

/// 束の外から与える生成材料。束の内容は描画シーン素材が、束縛の形はセットレイアウトが決める。
pub(in crate::renderer) struct シーン描画資源生成要求<'a> {
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    pub(in crate::renderer) 転送環境: &'a 転送実行環境,
    pub(in crate::renderer) セットレイアウト: &'a シーンセットレイアウト一式,
    pub(in crate::renderer) 描画シーン: &'a 描画シーン素材,
    pub(in crate::renderer) 材質id一覧: &'a [描画対象別の材質ID],
}

impl シーン描画資源 {
    /// 失敗したときは生成途中のGPU資源をすべて解放してからエラーを返すため、呼び出し元は自分が保持中の束をそのまま使い続けられる。
    pub(in crate::renderer) fn 生成する(
        device: &GPUデバイス, 要求: シーン描画資源生成要求<'_>
    ) -> Result<Self, レンダラーエラー> {
        let 材料 = チャンク描画資源生成材料 {
            メモリプロパティ: 要求.メモリプロパティ,
            転送環境: 要求.転送環境,
            セットレイアウト: 要求.セットレイアウト,
            材質id一覧: 要求.材質id一覧,
            動く個体一覧: 要求.描画シーン.動く個体一覧(),
        };
        let チャンク = チャンク描画資源::生成する(device, 材料, 起動シーンの束ID, 要求.描画シーン.描画対象一覧())?;
        let チャンク一覧 = vec![チャンク];
        let 描画対象数: usize = チャンク一覧.iter().map(チャンク描画資源::描画対象数).sum();
        Ok(Self {
            影のキャスター: crate::frame_input::影のキャスター指定::通常に描く,
            チャンク一覧,
            ジオメトリ入力作業領域: Vec::with_capacity(描画対象数),
            距離区分別のシャドウ入力作業領域: std::array::from_fn(|_| Vec::with_capacity(描画対象数)),
            点光源の影の作業領域: Vec::with_capacity(描画対象数),
            破棄待ち: Vec::new(),
            実破棄済みid一覧: Vec::new(),
            計数集計: super::描画計数集計::default(),
        })
    }
}
