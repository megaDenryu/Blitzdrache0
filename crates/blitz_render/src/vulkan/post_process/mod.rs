//! ポストプロセス段の資源一式(判断38・39・41): HDR中間画像・光のにじみピラミッド・光のにじみ一式・明るさの圧縮一式。
//! この4つはフレーム構成にポスト処理段階があるときだけ同時に存在し、一部だけが存在する状態を持たないため、
//! 4つの`Option`を並べる代わりに1つの型へまとめて有無を束ねる。保持側は`Option<ポスト処理一式>`を1つ持つ。
//! 生成手順は`create`、スワップチェーン寸法の変更への追従は`resize`、フレーム入力への平坦化は`inputs`にある。

mod create;
mod inputs;
mod resize;

pub(crate) use inputs::ポスト描画入力;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::bloom::光のにじみ一式;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tonemap::明るさの圧縮一式;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct ポスト処理一式 {
    hdrターゲット: HDRターゲット,
    光のにじみピラミッド: 光のにじみピラミッド,
    光のにじみ: 光のにじみ一式,
    明るさの圧縮: 明るさの圧縮一式,
}

impl ポスト処理一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        スワップチェーン画像形式: vk::Format,
        寸法: vk::Extent2D,
        シェーダー: &シェーダー束,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, スワップチェーン画像形式, 寸法, シェーダー)
    }

    /// 読み手である明るさの圧縮・光のにじみのパイプラインとディスクリプタを先に、読まれる側の画像を後に破棄する。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.明るさの圧縮.破棄する(device);
        self.光のにじみ.破棄する(device);
        self.光のにじみピラミッド.破棄する(device);
        self.hdrターゲット.破棄する(device);
    }
}
