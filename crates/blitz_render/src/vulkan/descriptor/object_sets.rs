//! 1つの束に属する描画対象の材質スロットぶんのディスクリプタセットを、その束専用のディスクリプタプールから確保して保持する。
//! 注意: プールの破棄がセットの解放を暗黙に行うため、束の解除はプール1つの破棄で完結する。セット添字もこの型の内側で閉じるため、他の束の追加・解除で添字がずれない。
//! セットを描画対象ではなく材質スロットごとに持つのは、1つのメッシュのプリミティブが材質スロットごとに違うテクスチャと係数を束ねるためである
//! (参照: `_doc/設計/マルチマテリアルと材質境界.md`「束縛バックエンドの移行境界」)。
//! 3つの添字から位置を導く配置は`slot_index`、割り当て済みのセットへの書き込みは`write`にある。

mod slot_index;
mod write;

use ash::vk;

use super::layout::ディスクリプタレイアウト;
use super::{pool, set};
use crate::error::レンダラーエラー;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::texture::マテリアルテクスチャ一式;
use crate::vulkan::uniform::フレームシェーダー定数一式;
use crate::vulkan::visible_id::可視ID列参照;
use slot_index::スロット別セット配置;

pub(crate) struct 描画対象ディスクリプタ参照<'a> {
    pub(crate) テクスチャ: &'a マテリアルテクスチャ一式,
    /// 材質レコード列を読むバッファと、そのバイト範囲。同じ描画対象の全スロットのセットが同じ値を持つ。
    pub(crate) 材質レコード: (vk::Buffer, vk::DeviceSize),
    /// 個体変換を読むバッファと、そのバイト範囲。個体が1体だけの対象も1要素ぶんの範囲を持つ専用のバッファを指す。
    pub(crate) 個体変換: (vk::Buffer, vk::DeviceSize),
    /// 可視ID列を読むバッファ。個体が1体だけの対象は束が共有する値0だけの列を指す。
    pub(crate) 可視id列: 可視ID列参照,
}

pub(crate) struct 描画対象ディスクリプタプール {
    pool: vk::DescriptorPool,
    set一覧: Vec<vk::DescriptorSet>,
    配置: スロット別セット配置,
}

impl 描画対象ディスクリプタプール {
    /// `対象別参照一覧`の並びが描画対象の添字であり、その内側の並びが材質スロットの添字である。
    pub(crate) fn 生成する(
        device: &ash::Device,
        レイアウト: &ディスクリプタレイアウト,
        対象別参照一覧: &[Vec<描画対象ディスクリプタ参照<'_>>],
        シェーダー定数: &フレームシェーダー定数一式,
        シャドウマップ: &シャドウマップ,
    ) -> Result<Self, レンダラーエラー> {
        let 対象別スロット数 = 対象別参照一覧.iter().map(Vec::len).collect::<Vec<usize>>();
        let 配置 = スロット別セット配置::生成する(&対象別スロット数);
        let pool = pool::生成する(device, 配置.合計スロット数())?;
        let set一覧 = match set::割り当てる(device, pool, レイアウト.handle(), 配置.セット数()) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                return Err(誤り);
            }
        };
        write::全セットの内容を書き込む(device, &set一覧, &配置, 対象別参照一覧, シェーダー定数, シャドウマップ);
        Ok(Self { pool, set一覧, 配置 })
    }

    /// 注意: 3つの添字は掛け合わせた1本の配列を参照するため、入れ替えると別の描画対象や別の材質のセットを束ねたまま描画が成立する。
    /// フレームスロット添字を別型にすることで、描画対象添字・材質スロット添字との入れ替えをコンパイルエラーにする。
    pub(crate) fn set(
        &self, 描画対象添字: usize, スロット添字: usize, フレーム添字: フレームスロット添字
    ) -> vk::DescriptorSet {
        let 位置 = match self.配置.位置(描画対象添字, スロット添字, フレーム添字) {
            Some(値) => 値,
            None => panic!("描画対象・材質スロット・フレームの添字がディスクリプタセットの配置の外だった"),
        };
        match self.set一覧.get(位置) {
            Some(set) => *set,
            None => panic!("ディスクリプタセット一覧が配置の示す位置を持たない"),
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}
