//! スキニングバッファの生成手順。途中失敗時は確保済みのバッファを台帳で巻き戻す。

mod attribute_bytes;

use ash::vk;

use self::attribute_bytes::属性をバイト列にする;
use super::スキニングバッファ;
use crate::error::レンダラーエラー;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan::allocator::巻き戻せる確保の台帳;
use crate::vulkan::geometry::bytes;
use crate::vulkan::transfer::ステージング経由の転送係;

const 行列バイト長: usize = 64;

impl スキニングバッファ {
    pub(in crate::vulkan::skinning) fn 生成する(
        転送係: ステージング経由の転送係<'_>,
        頂点一覧: &[頂点],
        素材: &スキンメッシュ素材,
    ) -> Result<Self, レンダラーエラー> {
        let 確保係 = 転送係.確保係();
        let mut 台帳 = 巻き戻せる確保の台帳::始める(確保係);

        let 頂点バイト列 = bytes::頂点をバイト列にする(頂点一覧);
        let レスト頂点 = 台帳.積む(転送係.データからデバイスローカルバッファを確保する(&頂点バイト列, vk::BufferUsageFlags::STORAGE_BUFFER))?;

        let 属性バイト列 = 属性をバイト列にする(素材);
        let 属性 = 台帳.積む(転送係.データからデバイスローカルバッファを確保する(&属性バイト列, vk::BufferUsageFlags::STORAGE_BUFFER))?;

        let 行列初期値 = vec![0u8; 素材.ジョイント数() * 行列バイト長];
        let 行列一覧 = 台帳.フレームスロットごとに積む(確保係.フレームスロットごとのホスト可視バッファを確保して書き込む(
            &行列初期値,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        ))?;

        // スキン済み頂点: コンピュートが書き、シーン/シャドウが頂点入力として読む(判断44の合流点)。
        let 出力サイズ = u64::try_from(頂点バイト列.len()).unwrap_or_else(|_| panic!("出力バッファ長がu64に収まらない"));
        let 出力 = 台帳.積む(確保係.デバイスローカルバッファを確保する(
            出力サイズ,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::VERTEX_BUFFER,
        ))?;

        Ok(Self {
            レスト頂点,
            属性,
            行列一覧,
            出力,
        })
    }
}
