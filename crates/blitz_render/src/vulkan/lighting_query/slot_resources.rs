//! 1つの進行中フレームスロットが専有する照明問い合わせ資源。触れるのはヘッダの定数バッファ・方向光レコード列・
//! 局所光レコード列の3本と、それらを結んだディスクリプタセットだけである。
//!
//! 不変条件: このスロットへ書けるのは、そのスロットの描画完了フェンスを通過した後だけである。
//! 全スロットで1つのセットとバッファを共有して毎フレーム上書きすると、GPUが読んでいる最中のバッファを
//! 書き換えることになる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。

use ash::vk;

use super::header_bytes;
use super::pack::{局所光列のバイト長, 方向光列のバイト長, 照明問い合わせのバイト列};
use super::writable_buffer::書き換えバッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::照明問い合わせのバッファ組;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct スロット資源 {
    pub(super) ヘッダ: 書き換えバッファ,
    pub(super) 方向光列: 書き換えバッファ,
    pub(super) 局所光列: 書き換えバッファ,
    pub(super) セット: vk::DescriptorSet,
}

impl スロット資源 {
    /// 3本のバッファを順に確保する。途中で失敗したら、そこまでに確保したぶんをその場で逆順に破棄する。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        セット: vk::DescriptorSet,
    ) -> Result<Self, レンダラーエラー> {
        let 定数用途 = vk::BufferUsageFlags::UNIFORM_BUFFER;
        let 列用途 = vk::BufferUsageFlags::STORAGE_BUFFER;
        let ヘッダ = 書き換えバッファ::生成する(device, メモリプロパティ, header_bytes::バイト長, 定数用途)?;
        let 方向光列 = match 書き換えバッファ::生成する(device, メモリプロパティ, 方向光列のバイト長, 列用途) {
            Ok(値) => 値,
            Err(誤り) => {
                ヘッダ.破棄する(device);
                return Err(誤り);
            }
        };
        let 局所光列 = match 書き換えバッファ::生成する(device, メモリプロパティ, 局所光列のバイト長, 列用途) {
            Ok(値) => 値,
            Err(誤り) => {
                方向光列.破棄する(device);
                ヘッダ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            ヘッダ,
            方向光列,
            局所光列,
            セット,
        })
    }

    /// ディスクリプタの結び方だけを知るモジュールへ渡す3本のハンドル。
    pub(super) fn バッファ組(&self) -> 照明問い合わせのバッファ組 {
        照明問い合わせのバッファ組 {
            ヘッダ: self.ヘッダ.buffer,
            方向光列: self.方向光列.buffer,
            局所光列: self.局所光列.buffer,
        }
    }

    pub(super) fn 書き込む(
        &self, device: &ash::Device, バイト列: &照明問い合わせのバイト列
    ) -> Result<(), レンダラーエラー> {
        self.ヘッダ.書き込む(device, &バイト列.ヘッダ)?;
        self.方向光列.書き込む(device, &バイト列.方向光列)?;
        self.局所光列.書き込む(device, &バイト列.局所光列)
    }

    /// 注意: ディスクリプタセットの解放はプールの破棄が暗黙に行うため、ここではバッファだけを破棄する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.局所光列.破棄する(device);
        self.方向光列.破棄する(device);
        self.ヘッダ.破棄する(device);
    }
}
