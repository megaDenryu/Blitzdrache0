//! 1つの束に属する描画対象のジオメトリのセット(set1)と材質のセット(set2)を、その束専用のディスクリプタプールから
//! 確保して保持する。注意: プールの破棄がセットの解放を暗黙に行うため、束の解除はプール1つの破棄で完結する。
//! セット添字もこの型の内側で閉じるため、他の束の追加・解除で添字がずれない。
//!
//! ジオメトリのセットを描画対象×フレームスロットで持つのは、可視ID列がフレームスロットごとに別のバッファだからである。
//! 材質のセットを材質スロットごとに持ちフレームスロットで分けないのは、材質レコードもテクスチャもフレームで変わらないためである
//! (参照: `_doc/設計/マルチマテリアルと材質境界.md`「束縛バックエンドの移行境界」)。
//! 2つの添字から位置を導く配置は`placement`、割り当て済みのセットへの書き込みは`write`にある。

mod placement;
mod write;

use ash::vk;

use super::シーンセットレイアウト一式;
use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::texture::マテリアルテクスチャ一式;
use crate::vulkan::visible_id::可視ID列参照;
use placement::セット配置;

/// 材質スロット1つぶんの、材質のセットが結ぶ資源。
pub(crate) struct 材質セット参照<'a> {
    pub(crate) テクスチャ: &'a マテリアルテクスチャ一式,
    /// 材質レコード列を読むバッファと、そのバイト範囲。同じ描画対象の全スロットのセットが同じ値を持つ。
    pub(crate) 材質レコード: (vk::Buffer, vk::DeviceSize),
}

/// 描画対象1つぶんの、ジオメトリのセットが結ぶ資源。
pub(crate) struct ジオメトリセット参照 {
    /// 個体レコードを読むバッファと、そのバイト範囲。個体が1体だけの対象も1要素ぶんの範囲を持つ専用のバッファを指す。
    pub(crate) 個体レコード: (vk::Buffer, vk::DeviceSize),
    /// 可視ID列を読むバッファ。個体が1体だけの対象は束が共有する値0だけの列を指す。
    pub(crate) 可視id列: 可視ID列参照,
}

pub(crate) struct 描画対象ディスクリプタプール {
    pool: vk::DescriptorPool,
    ジオメトリset一覧: Vec<vk::DescriptorSet>,
    材質set一覧: Vec<vk::DescriptorSet>,
    配置: セット配置,
}

impl 描画対象ディスクリプタプール {
    /// `対象別材質参照一覧`の並びが描画対象の添字であり、その内側の並びが材質スロットの添字である。
    /// `ジオメトリ参照一覧`の並びも同じ描画対象の添字である。
    pub(crate) fn 生成する(
        device: &ash::Device,
        レイアウト: &シーンセットレイアウト一式,
        ジオメトリ参照一覧: &[ジオメトリセット参照],
        対象別材質参照一覧: &[Vec<材質セット参照<'_>>],
    ) -> Result<Self, レンダラーエラー> {
        let 対象別スロット数 = 対象別材質参照一覧.iter().map(Vec::len).collect::<Vec<usize>>();
        let 配置 = セット配置::生成する(&対象別スロット数);
        let pool = write::プールを生成する(device, &配置)?;
        match write::割り当てて書き込む(device, pool, レイアウト, &配置, ジオメトリ参照一覧, 対象別材質参照一覧) {
            Ok((ジオメトリset一覧, 材質set一覧)) => Ok(Self {
                pool,
                ジオメトリset一覧,
                材質set一覧,
                配置,
            }),
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                Err(誤り)
            }
        }
    }

    /// 注意: フレームスロット添字を別型にすることで、描画対象添字との入れ替えをコンパイルエラーにする。
    pub(crate) fn ジオメトリセット(
        &self, 描画対象添字: usize, フレーム添字: フレームスロット添字
    ) -> vk::DescriptorSet {
        取り出す(&self.ジオメトリset一覧, self.配置.ジオメトリ位置(描画対象添字, フレーム添字))
    }

    pub(crate) fn 材質セット(&self, 描画対象添字: usize, スロット添字: usize) -> vk::DescriptorSet {
        取り出す(&self.材質set一覧, self.配置.材質位置(描画対象添字, スロット添字))
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}

/// 配置が返した位置のセット。位置が無いのも一覧がその位置を持たないのも、呼び出し元が渡した添字の誤りである。
fn 取り出す(set一覧: &[vk::DescriptorSet], 位置: Option<usize>) -> vk::DescriptorSet {
    match 位置.and_then(|位置| set一覧.get(位置)) {
        Some(set) => *set,
        None => panic!("ディスクリプタセットの添字が束の配置の外だった"),
    }
}
