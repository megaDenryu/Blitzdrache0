//! 読み戻しの受け皿4枚の確保と解放。担当する工程は「テクセル数の並びを受け取り、その順に受け皿を確保し、
//! 途中で失敗したらそれまでの確保を逆順に解放する」ことである。
//!
//! 4枚を1つの型で持つのは、コピーパスが4本そろって初めて意味を持ち、1枚だけ確保できた状態を呼び出し元へ
//! 見せる理由が無いためである。

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::readback_buffer::ベイク済み画像の読み戻しバッファ;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 受け皿一式 {
    バッファ一覧: [ベイク済み画像の読み戻しバッファ; 4],
}

impl 受け皿一式 {
    pub(super) fn 確保する(確保係: &GPU資源の確保係<'_>, テクセル数一覧: [usize; 4]) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let mut 作った: Vec<ベイク済み画像の読み戻しバッファ> = Vec::with_capacity(テクセル数一覧.len());
        for テクセル数 in テクセル数一覧 {
            match ベイク済み画像の読み戻しバッファ::生成する(確保係, テクセル数) {
                Ok(バッファ) => 作った.push(バッファ),
                Err(誤り) => {
                    while let Some(バッファ) = 作った.pop() {
                        バッファ.破棄する(device);
                    }
                    return Err(誤り);
                }
            }
        }
        let 件数 = 作った.len();
        Ok(Self {
            バッファ一覧: 作った
                .try_into()
                .unwrap_or_else(|_| panic!("読み戻しバッファを4本要求したのに{件数}本できた")),
        })
    }

    /// 並びは透過率・多重散乱・スカイビュー・空中遠近であり、確保に渡したテクセル数の並びと同じである。
    pub(super) fn 透過率(&self) -> &ベイク済み画像の読み戻しバッファ {
        &self.バッファ一覧[0]
    }

    pub(super) fn 多重散乱(&self) -> &ベイク済み画像の読み戻しバッファ {
        &self.バッファ一覧[1]
    }

    pub(super) fn スカイビュー(&self) -> &ベイク済み画像の読み戻しバッファ {
        &self.バッファ一覧[2]
    }

    pub(super) fn 空中遠近(&self) -> &ベイク済み画像の読み戻しバッファ {
        &self.バッファ一覧[3]
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for バッファ in &self.バッファ一覧 {
            バッファ.破棄する(device);
        }
    }
}
