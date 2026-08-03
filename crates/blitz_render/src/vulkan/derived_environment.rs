//! 遠方環境から導く3つの派生表現のGPU資源一式: 拡散照度の立方体画像・鏡面畳込みの立方体画像・反射率積分表と、
//! それぞれのディスクリプタと生成パイプラインである。生成の手順は`create`、パスの入力の組み立ては`pass_input`が持つ。
//!
//! 遠方環境一式と別の型にするのは、焼き直しの条件が別だからである。拡散と鏡面は遠方環境が焼き直されたときだけ
//! 焼き直せばよく、反射率積分表は大気にも時刻にも依らないため生成時の1度きりである。1つの型へ混ぜると
//! 3つの更新判断が同じ資源の中で絡む。
//!
//! 入力の遠方環境の画像は遠方環境一式が所有し、この型は2次元配列ビューを借りるだけである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「派生表現(3-Ib)」

pub(in crate::vulkan) mod copy_pass;
mod create;
mod cube_image;
mod descriptor;
mod descriptors;
mod images;
mod inputs;
pub(crate) mod pass;
mod pass_input;
mod pipelines;
mod probe;
mod table_descriptor;
mod table_image;

use ash::vk;

use crate::distant_environment::derived::鏡面畳込みの解像度;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan) use create::派生表現の解像度一式;
pub(in crate::vulkan) use cube_image::派生の立方体画像;
pub(crate) use inputs::派生表現の生成入力;
pub(in crate::vulkan) use pipelines::派生表現のシェーダー一式;
pub(crate) use probe::{派生表現をgpuで焼いて読み戻す, 派生表現を焼く条件};
pub(in crate::vulkan) use table_image::反射率積分表の画像;

pub(in crate::vulkan) struct 派生表現一式 {
    画像: images::派生表現の画像三点,
    ディスクリプタ: descriptors::派生表現のディスクリプタ三点,
    パイプライン: pipelines::派生表現のパイプライン三点,
    解像度: 派生表現の解像度一式,
}

impl 派生表現一式 {
    pub(in crate::vulkan) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 派生表現の解像度一式,
        遠方環境の配列ビュー: vk::ImageView,
        シェーダー: &派生表現のシェーダー一式<'_>,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 解像度, 遠方環境の配列ビュー, シェーダー)
    }

    pub(in crate::vulkan) fn 拡散照度画像(&self) -> &派生の立方体画像 {
        &self.画像.拡散照度
    }

    pub(in crate::vulkan) fn 鏡面畳込み画像(&self) -> &派生の立方体画像 {
        &self.画像.鏡面畳込み
    }

    pub(in crate::vulkan) fn 反射率積分表の画像(&self) -> &反射率積分表の画像 {
        &self.画像.反射率積分表
    }

    /// 鏡面畳込みの段構成。焼く段の列挙と段ごとの粗さがここから決まる。
    pub(in crate::vulkan) fn 鏡面畳込みの解像度(&self) -> 鏡面畳込みの解像度 {
        self.解像度.鏡面畳込み
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan) fn 破棄する(&self, device: &GPUデバイス) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.画像.破棄する(device);
    }
}
