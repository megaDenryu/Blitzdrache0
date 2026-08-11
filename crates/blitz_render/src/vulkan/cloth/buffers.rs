//! 布シミュレーションのバッファ群(判断54)。粒子・前位置・隣接拘束・空間グリッド2本・
//! 布頂点(STORAGE|VERTEX)・インデックス・アタッチ対応と、進行中フレーム2重の
//! 介入キュー・定数UBO(ホスト可視)。生成手順は`create`にある。

mod create;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{フレームスロットごとのバッファ, 専用メモリ付きバッファ};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 布バッファ {
    pub(super) 粒子: 専用メモリ付きバッファ,
    pub(super) 前位置: 専用メモリ付きバッファ,
    pub(super) 隣接: 専用メモリ付きバッファ,
    pub(super) セルカウント: 専用メモリ付きバッファ,
    pub(super) セル格納: 専用メモリ付きバッファ,
    pub(super) 布頂点: 専用メモリ付きバッファ,
    pub(super) インデックス: 専用メモリ付きバッファ,
    pub(super) アタッチ: 専用メモリ付きバッファ,
    pub(super) 介入一覧: フレームスロットごとのバッファ,
    pub(super) 定数一覧: フレームスロットごとのバッファ,
}

pub(super) use create::生成する;

impl 布バッファ {
    /// 前提: 呼び出しはフェンス待ち後(このスロットの前回GPU使用の完了後。判断24と同じ規律)。
    pub(super) fn 介入を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        バイト列: &[u8],
    ) -> Result<(), レンダラーエラー> {
        self.介入一覧.スロットの中身を書き換える(device, フレーム添字, バイト列)
    }

    /// 前提: 同上。
    pub(super) fn 定数を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        バイト列: &[u8],
    ) -> Result<(), レンダラーエラー> {
        self.定数一覧.スロットの中身を書き換える(device, フレーム添字, バイト列)
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        let 固定一覧 = [
            &self.粒子,
            &self.前位置,
            &self.隣接,
            &self.セルカウント,
            &self.セル格納,
            &self.布頂点,
            &self.インデックス,
            &self.アタッチ,
        ];
        for バッファ in 固定一覧 {
            バッファ.破棄する(device);
        }
        self.介入一覧.破棄する(device);
        self.定数一覧.破棄する(device);
    }
}
