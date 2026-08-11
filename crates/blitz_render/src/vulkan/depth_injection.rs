//! 検収が与えた合成深度を保持し、毎フレームそれを深度画像へ書き戻す資源。担当するのは、CPU正本が焼いた深度1枚を
//! ホスト可視バッファへ写して持ち続けることと、そのバッファから深度画像への転送をパス1本として差し出すことである。
//!
//! バッファを持ち続けるのは、深度プリパスが毎フレーム深度を上書きするためである。1度だけ書き込む形にすると、
//! 次のフレームには本番のジオメトリの深度へ戻ってしまう。
//!
//! この資源が在るのは検収の実行だけである。本番の起動では`レンダラー`が`None`を持ち、パスも1本積まれない。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」

mod pass;

use ash::vk;

pub(crate) use pass::合成深度の注入を作る;

use crate::error::レンダラーエラー;
use crate::local_visibility::深度画像;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

/// 合成深度1枚ぶんのホスト可視バッファと、その深度が焼かれた寸法。
pub(crate) struct 合成深度の注入一式 {
    バッファ: 専用メモリ付きバッファ,
    寸法: vk::Extent2D,
}

impl 合成深度の注入一式 {
    /// 前提: 深度画像の寸法がスワップチェーンの寸法と一致することは呼び出し元が確かめている。
    pub(crate) fn 生成する(確保係: &GPU資源の確保係<'_>, 深度画像: &深度画像) -> Result<Self, レンダラーエラー> {
        let バッファ =
            確保係.ホスト可視バッファを確保して書き込む(&バイト列へ写す(深度画像), vk::BufferUsageFlags::TRANSFER_SRC)?;
        Ok(Self {
            バッファ,
            寸法: vk::Extent2D {
                width: 深度画像.寸法().幅(),
                height: 深度画像.寸法().高さ(),
            },
        })
    }

    pub(crate) fn 描画入力を作る(&self) -> 合成深度の注入入力 {
        合成深度の注入入力 {
            バッファ: self.バッファ.バッファのハンドル(),
            寸法: self.寸法,
        }
    }

    /// 前提: 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}

/// 転送パスが要るハンドルと寸法。
#[derive(Clone, Copy)]
pub(crate) struct 合成深度の注入入力 {
    pub(crate) バッファ: vk::Buffer,
    pub(crate) 寸法: vk::Extent2D,
}

/// 深度形式がD32_SFLOATであるため、単精度をそのままリトルエンディアンで並べた列が転送の中身になる。
fn バイト列へ写す(深度画像: &深度画像) -> Vec<u8> {
    let 寸法 = 深度画像.寸法();
    let mut バイト列 = Vec::with_capacity(寸法.画素数() * 4);
    for 縦 in 0..寸法.高さ() {
        for 横 in 0..寸法.幅() {
            バイト列.extend_from_slice(&深度画像.深度(横, 縦).to_le_bytes());
        }
    }
    バイト列
}
