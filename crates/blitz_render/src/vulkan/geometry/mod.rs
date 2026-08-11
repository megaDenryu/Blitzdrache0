//! glTFメッシュ用の頂点・インデックスバッファ。ステージング+デバイスローカル転送で
//! 確保する(判断20)。アセットホットリロード(`シーンを差し替える`)のたびに破棄・再生成する。
//! インデックスの総数を持たないのは、描画発行がプリミティブごとに自分のインデックス区間を持つためである
//! (参照: `_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」)。

pub(crate) mod bytes;
pub(crate) mod upload;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vertex::頂点;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::allocator::専用メモリ付きバッファ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct ジオメトリバッファ {
    頂点: 専用メモリ付きバッファ,
    索引: 専用メモリ付きバッファ,
}

impl ジオメトリバッファ {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &転送実行環境,
        頂点一覧: &[頂点],
        インデックス一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 頂点バイト列 = bytes::頂点をバイト列にする(頂点一覧);
        let 頂点バッファ =
            upload::ステージング経由でアップロードする(確保係, 転送環境, &頂点バイト列, vk::BufferUsageFlags::VERTEX_BUFFER)?;

        let インデックスバイト列 = bytes::インデックスをバイト列にする(インデックス一覧);
        let インデックスバッファ = match upload::ステージング経由でアップロードする(
            確保係,
            転送環境,
            &インデックスバイト列,
            vk::BufferUsageFlags::INDEX_BUFFER,
        ) {
            Ok(結果) => 結果,
            Err(誤り) => {
                頂点バッファ.破棄する(device);
                return Err(誤り);
            }
        };

        Ok(Self {
            頂点: 頂点バッファ,
            索引: インデックスバッファ,
        })
    }

    pub(crate) fn 頂点バッファ(&self) -> vk::Buffer {
        self.頂点.バッファのハンドル()
    }

    pub(crate) fn インデックスバッファ(&self) -> vk::Buffer {
        self.索引.バッファのハンドル()
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.索引.破棄する(device);
        self.頂点.破棄する(device);
    }
}
