//! 検査が与える遠方環境の中身を載せたホスト可視バッファの所有者。担うのは、確保・中身の書き込み・転送元としての
//! 貸し出し・破棄である。バイト並びは`environment_bytes`、転送パスの宣言は`copy_pass`が持つ。
//!
//! 大気から焼くのでなくホストから与えるのは、畳み込みの正しさを解析解と突き合わせるためである。
//! 定数の環境なら拡散照度は円周率×放射輝度、鏡面畳込みはどの粗さ段も放射輝度そのものになる。

use ash::vk;

use crate::distant_environment::derived::遠方環境の内容;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 遠方環境の書き込みバッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 遠方環境の書き込みバッファ {
    pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>, 内容: &遠方環境の内容) -> Result<Self, レンダラーエラー> {
        let バイト列 = super::environment_bytes::半精度のバイト列へ詰める(内容);
        let バッファ = 確保係.ホスト可視バッファを確保して書き込む(&バイト列, vk::BufferUsageFlags::TRANSFER_SRC)?;
        Ok(Self { バッファ })
    }

    pub(super) fn handle(&self) -> vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
