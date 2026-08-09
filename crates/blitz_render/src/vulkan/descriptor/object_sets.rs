//! 1つの束に属する描画対象のジオメトリのセット(set1)を、その束専用のディスクリプタプールから確保して保持する。
//! 注意: プールの破棄がセットの解放を暗黙に行うため、束の解除はプール1つの破棄で完結する。
//! セット添字もこの型の内側で閉じるため、他の束の追加・解除で添字がずれない。
//!
//! ジオメトリのセットを描画対象×フレームスロットで持つのは、可視ID列がフレームスロットごとに別のバッファだからである。
//! 材質のセット(set2)は束が持たない。資源表世代が1つだけ持ち、パスの先頭で1回束縛する
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。
//! 確保と書き込みの局面は`create`にある。

mod create;

use ash::vk;

use crate::vulkan::instance_transform::個体レコード参照;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::visible_id::可視ID列参照;

/// 描画対象1つぶんの、ジオメトリのセットが結ぶ資源。
pub(crate) struct ジオメトリセット参照 {
    /// 個体レコードを読むバッファ。個体が1体だけの対象も1要素ぶんの範囲を持つ専用のバッファを指し、
    /// 動く個体を宣言した対象だけがフレームスロットごとに別のバッファを指す。
    pub(crate) 個体レコード: 個体レコード参照,
    /// 可視ID列を読むバッファ。個体が1体だけの対象は束が共有する値0だけの列を指す。
    pub(crate) 可視id列: 可視ID列参照,
}

pub(crate) struct 描画対象ディスクリプタプール {
    pool: vk::DescriptorPool,
    /// 描画対象の順、その中でフレームスロットの順に並ぶ。
    ジオメトリset一覧: Vec<vk::DescriptorSet>,
    描画対象数: usize,
}

impl 描画対象ディスクリプタプール {
    fn 束ねる(pool: vk::DescriptorPool, ジオメトリset一覧: Vec<vk::DescriptorSet>, 描画対象数: usize) -> Self {
        Self {
            pool,
            ジオメトリset一覧,
            描画対象数,
        }
    }

    /// 注意: フレームスロット添字を別型にすることで、描画対象添字との入れ替えをコンパイルエラーにする。
    pub(crate) fn ジオメトリセット(
        &self, 描画対象添字: usize, フレーム添字: フレームスロット添字
    ) -> vk::DescriptorSet {
        assert!(描画対象添字 < self.描画対象数, "ジオメトリのセットの描画対象添字が束の配置の外だった");
        match self.ジオメトリset一覧.get(位置を求める(描画対象添字, フレーム添字)) {
            Some(set) => *set,
            None => panic!("ディスクリプタセット一覧が描画対象添字の示す位置を持たない"),
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}

fn 位置を求める(描画対象添字: usize, フレーム添字: フレームスロット添字) -> usize {
    描画対象添字
        .checked_mul(進行中フレーム数)
        .and_then(|積| 積.checked_add(フレーム添字.配列添字()))
        .unwrap_or_else(|| panic!("ジオメトリのセットの位置がusizeを超えた"))
}
