//! ブルーム用の1/2解像度中間画像2枚(判断39)。aは抽出結果と縦ぼかし後の最終結果、
//! bは横ぼかしの中間結果を持つピンポン構成。形式・用途はHDR中間画像と同一のため
//! `HDRターゲット`を再利用する。スワップチェーン再構築と連動して作り直す。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::hdr_target::HDRターゲット;

pub(crate) struct ブルームターゲット {
    pub(crate) a: HDRターゲット,
    pub(crate) b: HDRターゲット,
    pub(crate) 寸法: vk::Extent2D,
}

impl ブルームターゲット {
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        フル解像度: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        let 寸法 = vk::Extent2D {
            width: (フル解像度.width / 2).max(1),
            height: (フル解像度.height / 2).max(1),
        };
        let a = HDRターゲット::生成する(device, メモリプロパティ, 寸法)?;
        let b = match HDRターゲット::生成する(device, メモリプロパティ, 寸法) {
            Ok(b) => b,
            Err(誤り) => {
                a.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self { a, b, 寸法 })
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        self.a.破棄する(device);
        self.b.破棄する(device);
    }
}
