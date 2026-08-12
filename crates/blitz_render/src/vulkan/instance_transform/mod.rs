//! 描画対象の個体変換をGPUへ載せる静的なストレージバッファ。個体が1体だけの対象も1要素のこのバッファを持ち、
//! 他の資源の先頭を個体変換として読む経路は持たない(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」)。
//! 束の読込時に一度だけ書き、以後変えない。可視判定やLOD選択で書き直さないため、可視個体の選別のたびに全個体を転送することがない。
//! 書いた後は頂点シェーダーが個体数ぶん読むだけであるため、頂点・インデックスと同じくステージング経由でデバイスローカルへ載せる。
//! ホスト可視メモリに置くと大個体数の頂点シェーダーがホスト側のメモリを読み続けることになり、物量計測の折れ点が植生の負荷でなくメモリ配置に支配される。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」

pub(crate) mod bytes;
pub(crate) mod content;
mod dynamic;
mod dynamic_allocation;
#[cfg(test)]
mod layout_tests;
mod reference;

pub(crate) use dynamic::動く個体の変換バッファ;
pub(crate) use reference::個体レコード参照;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::専用メモリ付きバッファ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;
use content::個体変換内容;

pub(crate) struct 個体変換バッファ {
    バッファ: 専用メモリ付きバッファ,
    /// バッファが保持する個体変換のバイト数。ディスクリプタの範囲に使う。
    pub(crate) 範囲: vk::DeviceSize,
}

impl 個体変換バッファ {
    pub(crate) fn 生成する(
        転送係: ステージング経由の転送係<'_>, 内容一覧: &[個体変換内容]
    ) -> Result<Self, レンダラーエラー> {
        let mut バイト列 = Vec::with_capacity(内容一覧.len() * bytes::バイト長);
        for 内容 in 内容一覧 {
            バイト列.extend_from_slice(&bytes::バイト列にする(内容));
        }
        let バッファ = 転送係.データからデバイスローカルバッファを確保する(&バイト列, vk::BufferUsageFlags::STORAGE_BUFFER)?;
        let 範囲 = u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("個体変換バッファの長さがu64に収まらない"));
        Ok(Self { バッファ, 範囲 })
    }

    pub(crate) fn 参照(&self) -> 個体レコード参照 {
        個体レコード参照::全スロット共通のバッファから生成する(self.バッファ.バッファのハンドル(), self.範囲)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
