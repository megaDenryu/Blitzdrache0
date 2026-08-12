//! 粒子ストレージバッファ(デバイスローカル、位置vec4+速度vec4/粒子)。
//! 初回構築時に1回だけ確保し、以後は毎フレーム使い回す(判断29・レンダーグラフ.md
//! 「仮想リソース」のグラフ管理の項)。ステージング転送での初期化はジオメトリバッファと
//! 同じ転送基盤(`vulkan::transfer::ステージング経由の転送係`)を再利用する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::particle_material::粒子素材;
use crate::vulkan::allocator::専用メモリ付きバッファ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;

pub(crate) struct 粒子バッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 粒子バッファ {
    pub(crate) fn バッファのハンドル(&self) -> vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    pub(crate) fn 生成する(転送係: ステージング経由の転送係<'_>, 素材: &粒子素材) -> Result<Self, レンダラーエラー> {
        let バッファ =
            転送係.データからデバイスローカルバッファを確保する(&素材.初期バイト列, vk::BufferUsageFlags::STORAGE_BUFFER)?;
        Ok(Self { バッファ })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
