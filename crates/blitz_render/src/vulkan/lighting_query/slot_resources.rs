//! 1つの進行中フレームスロットが専有する照明問い合わせ資源。触れるのはヘッダの定数バッファ・方向光レコード列・
//! 局所光レコード列の3本と、それらを結んだディスクリプタセットだけである。
//!
//! 不変条件: このスロットへ書けるのは、そのスロットの描画完了フェンスを通過した後だけである。
//! 全スロットで1つのセットとバッファを共有して毎フレーム上書きすると、GPUが読んでいる最中のバッファを
//! 書き換えることになる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。

use ash::vk;

use super::cluster_buffers::クラスタ格子の資源;
use super::header_bytes;
use super::pack::{局所光列のバイト長, 方向光列のバイト長, 照明問い合わせのバイト列};
use super::writable_buffer::書き換えバッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::descriptor::照明問い合わせのバッファ組;
use crate::vulkan::descriptor::照明問い合わせの割り当て済みセット;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct スロット資源 {
    pub(super) ヘッダ: 書き換えバッファ,
    pub(super) 方向光列: 書き換えバッファ,
    pub(super) 局所光列: 書き換えバッファ,
    pub(super) クラスタ格子: クラスタ格子の資源, // 選別のコンピュートだけが書く2本。CPUからの書き込みの口を持たない。
    pub(super) セット: 照明問い合わせの割り当て済みセット,
}

impl スロット資源 {
    /// 3本のバッファを順に確保する。途中で失敗したら、そこまでに確保したぶんをその場で逆順に破棄する。
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>, セット: 照明問い合わせの割り当て済みセット
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 定数用途 = vk::BufferUsageFlags::UNIFORM_BUFFER;
        let 列用途 = vk::BufferUsageFlags::STORAGE_BUFFER;
        let ヘッダ = 書き換えバッファ::生成する(確保係, header_bytes::バイト長, 定数用途)?;
        let 方向光列 = match 書き換えバッファ::生成する(確保係, 方向光列のバイト長, 列用途) {
            Ok(値) => 値,
            Err(誤り) => {
                ヘッダ.破棄する(device);
                return Err(誤り);
            }
        };
        let 局所光列 = match 書き換えバッファ::生成する(確保係, 局所光列のバイト長, 列用途) {
            Ok(値) => 値,
            Err(誤り) => {
                方向光列.破棄する(device);
                ヘッダ.破棄する(device);
                return Err(誤り);
            }
        };
        let クラスタ格子 = match クラスタ格子の資源::生成する(確保係) {
            Ok(値) => 値,
            Err(誤り) => {
                局所光列.破棄する(device);
                方向光列.破棄する(device);
                ヘッダ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            ヘッダ,
            方向光列,
            局所光列,
            クラスタ格子,
            セット,
        })
    }

    /// ディスクリプタの結び方だけを知るモジュールへ渡す5本のハンドル。
    pub(super) fn バッファ組(&self) -> 照明問い合わせのバッファ組 {
        照明問い合わせのバッファ組 {
            ヘッダ: self.ヘッダ.バッファのハンドル(),
            方向光列: self.方向光列.バッファのハンドル(),
            局所光列: self.局所光列.バッファのハンドル(),
            クラスタ格子: self.クラスタ格子.格子のバッファ(),
            クラスタ光添字列: self.クラスタ格子.光添字列のバッファ(),
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
        self.クラスタ格子.破棄する(device);
        self.局所光列.破棄する(device);
        self.方向光列.破棄する(device);
        self.ヘッダ.破棄する(device);
    }
}
