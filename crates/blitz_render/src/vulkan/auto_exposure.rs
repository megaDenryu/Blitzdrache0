//! 自動露出の資源一式。担当するのは、世界の宣言(`crate::auto_exposure::自動露出の設定`)をGPUの資源へ落とし、
//! 毎フレームのパスが要るハンドルと定数を1つの入力へまとめることである。
//!
//! ポスト処理一式と同時に作るのは、集計が読むのが明るさの圧縮前のHDR中間画像であり、更新した露出状態を読むのが
//! 明るさの圧縮だからである。露出方式が時刻別固定の世界でも資源を作るのは、作るかどうかを方式で分けると
//! 明るさの圧縮のディスクリプタが世界ごとに違う形になり、束縛の一致条件が1つ増えるためである。
//! パスを積むかどうかだけが方式で分かれる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「順3-IIの実装設計」

mod bind;
mod descriptor;
mod input;
mod pipelines;
mod readback;
mod storage;
mod storage_buffer;

pub(crate) use input::自動露出描画入力;
pub(crate) use storage::ヒストグラムのバイト数;

use ash::vk;

use crate::auto_exposure::{自動露出のシェーダー一式, 自動露出の設定};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct 自動露出一式 {
    バッファ: storage::自動露出のバッファ一式,
    ディスクリプタ: descriptor::自動露出のディスクリプタ,
    パイプライン: pipelines::自動露出のパイプライン一式,
    設定: 自動露出の設定,
}

impl 自動露出一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &転送実行環境,
        シェーダー: &自動露出のシェーダー一式,
        設定: 自動露出の設定,
        hdrビュー: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let バッファ = storage::自動露出のバッファ一式::生成する(確保係, 転送環境, &設定.境界の線形輝度)?;
        let ディスクリプタ = match descriptor::自動露出のディスクリプタ::生成する(device) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                バッファ.破棄する(device);
                return Err(誤り);
            }
        };
        let パイプライン =
            match pipelines::自動露出のパイプライン一式::生成する(確保係, &ディスクリプタ, &シェーダー.集計, &シェーダー.導出と適応)
            {
                Ok(パイプライン) => パイプライン,
                Err(誤り) => {
                    ディスクリプタ.破棄する(device);
                    バッファ.破棄する(device);
                    return Err(誤り);
                }
            };
        let 一式 = Self {
            バッファ,
            ディスクリプタ,
            パイプライン,
            設定,
        };
        一式.資源を束縛する(device, hdrビュー);
        Ok(一式)
    }

    /// 明るさの圧縮のディスクリプタが束縛する、GPU上の露出状態。
    pub(crate) fn 露出状態バッファ(&self) -> vk::Buffer {
        self.バッファ.露出状態.handle()
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.バッファ.破棄する(device);
    }
}
