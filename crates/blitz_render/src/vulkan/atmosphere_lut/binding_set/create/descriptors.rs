//! 5つのディスクリプタの生成と、途中で失敗したときの巻き戻し。担当する工程は「基盤資源のビューとシェーダー定数を
//! 受け取り、5つそろったディスクリプタを返す」ことである。触れるのはディスクリプタのハンドルだけであり、
//! パイプラインは1本も作らない。
//!
//! 2枚のスカイビューと空中遠近は同じ型であり書き込み先だけが違うため、3つの生成は`march_series`が並びで扱う。

mod march_series;

use ash::vk;

use march_series::経路生成を順に作る;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::base_resources::大気のベイク済み画像の基盤資源;
use crate::vulkan::atmosphere_lut::march_descriptor::経路生成ディスクリプタ;
use crate::vulkan::atmosphere_lut::multiscatter_descriptor::{多重散乱の束縛先, 多重散乱ディスクリプタ};
use crate::vulkan::atmosphere_lut::transmittance_descriptor::透過率ディスクリプタ;
use crate::vulkan::sync::進行中フレーム数;

/// 5つそろったディスクリプタ。そろってから初めてパイプラインを作る。
pub(super) struct ディスクリプタ五点 {
    pub(super) 透過率: 透過率ディスクリプタ,
    pub(super) 多重散乱: 多重散乱ディスクリプタ,
    pub(super) スカイビュー: 経路生成ディスクリプタ,
    pub(super) 遠方環境用スカイビュー: 経路生成ディスクリプタ,
    pub(super) 空中遠近: 経路生成ディスクリプタ,
}

impl ディスクリプタ五点 {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.空中遠近.破棄する(device);
        self.遠方環境用スカイビュー.破棄する(device);
        self.スカイビュー.破棄する(device);
        self.多重散乱.破棄する(device);
        self.透過率.破棄する(device);
    }
}

pub(super) fn 作る(
    確保係: &GPU資源の確保係<'_>,
    基盤: &大気のベイク済み画像の基盤資源,
) -> Result<ディスクリプタ五点, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let シェーダー定数一覧 = 基盤.シェーダー定数一覧();
    let 透過率 = 透過率ディスクリプタ::生成する(device, シェーダー定数一覧, 基盤.透過率.画像ビュー)?;
    let 多重散乱 = match 多重散乱を作る(確保係, 基盤, &シェーダー定数一覧) {
        Ok(値) => 値,
        Err(誤り) => {
            透過率.破棄する(device);
            return Err(誤り);
        }
    };
    let 書き込み先一覧 = [
        基盤.スカイビュー.画像ビュー,
        基盤.遠方環境用スカイビュー.画像ビュー,
        基盤.空中遠近.画像ビュー,
    ];
    match 経路生成を順に作る(確保係, 基盤, &シェーダー定数一覧, 書き込み先一覧) {
        Ok([スカイビュー, 遠方環境用スカイビュー, 空中遠近]) => Ok(ディスクリプタ五点 {
            透過率,
            多重散乱,
            スカイビュー,
            遠方環境用スカイビュー,
            空中遠近,
        }),
        Err(誤り) => {
            多重散乱.破棄する(device);
            透過率.破棄する(device);
            Err(誤り)
        }
    }
}

fn 多重散乱を作る(
    確保係: &GPU資源の確保係<'_>,
    基盤: &大気のベイク済み画像の基盤資源,
    シェーダー定数一覧: &[vk::Buffer; 進行中フレーム数],
) -> Result<多重散乱ディスクリプタ, レンダラーエラー> {
    多重散乱ディスクリプタ::生成する(
        確保係,
        多重散乱の束縛先 {
            シェーダー定数一覧,
            透過率ビュー: 基盤.透過率.画像ビュー,
            多重散乱ビュー: 基盤.多重散乱.画像ビュー,
        },
    )
}
