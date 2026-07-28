//! 1つの描画対象が個体変換をどこから読むかを表す判別共用体と、その確保。
//! 触れるのは自分が確保したバッファだけであり、ディスクリプタが指すバッファと範囲を返す読み取りに閉じる。
//!
//! 個体が1体だけの対象で専用バッファを確保しないのは、描画対象ユニフォームの先頭112バイトが個体変換1件と
//! 同じ配置であり、そこを指せば足りるためである。全対象へ1件のストレージバッファを足すと、既存の描画対象の
//! GPU確保数が1つずつ増える(参照: `_doc/設計/植生インスタンスと物量計測.md`「レンダラーの資源配置」)。

use ash::vk;
use blitz_math::{ローカル, ワールド, 変換};

use crate::error::レンダラーエラー;
use crate::vulkan::instance_transform::content::個体変換内容;
use crate::vulkan::instance_transform::{bytes, 個体変換バッファ};
use crate::vulkan::object_uniform::描画対象ユニフォーム;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) enum 個体変換の出どころ {
    /// 個体が1体だけの対象。描画対象ユニフォームの先頭を個体変換1件として読む。
    描画対象ユニフォームの先頭,
    専用バッファ(個体変換バッファ),
}

impl 個体変換の出どころ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        個体変換一覧: &[変換<ローカル, ワールド>],
    ) -> Result<Self, レンダラーエラー> {
        if 個体変換一覧.len() <= 1 {
            return Ok(Self::描画対象ユニフォームの先頭);
        }
        let mut 内容一覧 = Vec::with_capacity(個体変換一覧.len());
        for 変換 in 個体変換一覧 {
            内容一覧.push(個体変換内容::変換から作る(*変換)?);
        }
        Ok(Self::専用バッファ(個体変換バッファ::生成する(
            device,
            メモリプロパティ,
            &内容一覧,
        )?))
    }

    /// ディスクリプタのbinding6へ結ぶバッファとバイト範囲。
    pub(super) fn ディスクリプタ参照(&self, ユニフォーム: &描画対象ユニフォーム) -> (vk::Buffer, vk::DeviceSize) {
        match self {
            Self::描画対象ユニフォームの先頭 => {
                let 範囲 = u64::try_from(bytes::バイト長).unwrap_or_else(|_| panic!("個体変換のバイト長がu64に収まらない"));
                (ユニフォーム.buffer, 範囲)
            }
            Self::専用バッファ(バッファ) => (バッファ.buffer, バッファ.範囲),
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        match self {
            Self::描画対象ユニフォームの先頭 => {}
            Self::専用バッファ(バッファ) => バッファ.破棄する(device),
        }
    }
}
