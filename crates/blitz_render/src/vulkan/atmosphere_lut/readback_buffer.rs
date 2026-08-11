//! ベイク済み画像1枚ぶんを受けるホスト可視バッファ。保持・コピー先としての貸し出し・マッピングして読むことを担う。
//! 確保の手順は`create`、半精度のビット列を単精度へ開く工程は`half_float`が持つ。
//!
//! 1テクセルの成分数を保持するのは、4成分の画像のほかに2成分の反射率積分表を受けるためである。
//! 成分数を取り違えるとバッファの容量とコピーの量が食い違い、確保の外へ書き込むことになる。

mod create;
mod half_float;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

use half_float::{二成分を単精度へ開く, 単精度へ開く};

pub(in crate::vulkan) struct ベイク済み画像の読み戻しバッファ {
    バッファ: 専用メモリ付きバッファ,
    テクセル数: usize,
    テクセルのバイト数: usize,
}

impl ベイク済み画像の読み戻しバッファ {
    pub(in crate::vulkan) fn 生成する(確保係: &GPU資源の確保係<'_>, テクセル数: usize) -> Result<Self, レンダラーエラー> {
        create::四成分で生成する(確保係, テクセル数)
    }

    /// 2成分の半精度画像を受ける。反射率積分表だけがこちらを使う。
    pub(in crate::vulkan) fn 二成分で生成する(
        確保係: &GPU資源の確保係<'_>, テクセル数: usize
    ) -> Result<Self, レンダラーエラー> {
        create::二成分で生成する(確保係, テクセル数)
    }

    pub(in crate::vulkan) fn handle(&self) -> vk::Buffer {
        self.バッファ.バッファ()
    }

    /// 前提: 呼び出し元はこのバッファへのコピーがフェンス待機で完了済みであることを保証する。
    pub(in crate::vulkan) fn 読み取る(&self, device: &ash::Device) -> Result<Vec<[f32; 4]>, レンダラーエラー> {
        Ok(単精度へ開く(&self.バイト列を写す(device)?))
    }

    /// 2成分で確保したバッファを読む。
    /// 前提: 呼び出し元はこのバッファへのコピーがフェンス待機で完了済みであることを保証する。
    pub(in crate::vulkan) fn 二成分で読み取る(&self, device: &ash::Device) -> Result<Vec<[f32; 2]>, レンダラーエラー> {
        Ok(二成分を単精度へ開く(&self.バイト列を写す(device)?))
    }

    fn バイト列を写す(&self, device: &ash::Device) -> Result<Vec<u8>, レンダラーエラー> {
        self.バッファ
            .ホスト可視のバイト列を写し取る(device, self.テクセル数 * self.テクセルのバイト数)
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(in crate::vulkan) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
