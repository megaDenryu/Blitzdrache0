//! ポストプロセス段の資源一式(判断38・39・41): HDR中間画像・光のにじみピラミッド・光のにじみ一式・明るさの圧縮一式・自動露出一式。
//! この5つはフレーム構成にポスト処理段階があるときだけ同時に存在し、一部だけが存在する状態を持たないため、
//! 5つの`Option`を並べる代わりに1つの型へまとめて有無を束ねる。保持側は`Option<ポスト処理一式>`を1つ持つ。
//! 自動露出をここへ入れるのは、集計が読むのが明るさの圧縮前のHDR中間画像であり、更新した露出状態を読むのが明るさの圧縮だからである。
//! 生成手順は`create`、スワップチェーン寸法の変更への追従は`resize`、フレーム入力への平坦化は`inputs`にある。

mod create;
mod inputs;
mod resize;
mod sized_images;

pub(crate) use inputs::ポスト描画入力;

use ash::vk;

use crate::auto_exposure::自動露出の設定;
use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::auto_exposure::自動露出一式;
use crate::vulkan::bloom::光のにじみ一式;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tonemap::明るさの圧縮一式;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct ポスト処理一式 {
    hdrターゲット: HDRターゲット,
    光のにじみピラミッド: 光のにじみピラミッド,
    光のにじみ: 光のにじみ一式,
    明るさの圧縮: 明るさの圧縮一式,
    自動露出: 自動露出一式,
}

impl ポスト処理一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        スワップチェーン画像形式: vk::Format,
        寸法: vk::Extent2D,
        シェーダー: &シェーダー束,
        転送環境: &転送実行環境,
        自動露出の設定: 自動露出の設定,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(create::生成材料 {
            device,
            メモリプロパティ,
            スワップチェーン画像形式,
            寸法,
            シェーダー,
            転送環境,
            自動露出の設定,
        })
    }

    /// GPU上の自動露出の中身を検収へ渡す。前提: 呼び出し元がGPUの全作業完了を待ってから呼ぶ。
    pub(crate) fn 自動露出の観測を読み戻す(
        &self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
    ) -> Result<crate::auto_exposure::自動露出の観測, レンダラーエラー> {
        self.自動露出.観測を読み戻す(device, メモリプロパティ, 転送環境)
    }

    /// 読み手である明るさの圧縮・光のにじみのパイプラインとディスクリプタを先に、読まれる側の画像を後に破棄する。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この束はその1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.自動露出.破棄する(device);
        self.明るさの圧縮.破棄する(device);
        self.光のにじみ.破棄する(device);
        self.光のにじみピラミッド.破棄する(device);
        self.hdrターゲット.破棄する(device);
    }
}
