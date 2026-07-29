//! 3つのディスクリプタの生成と、途中で失敗したときの巻き戻し。担当する工程は「基盤資源のビューとユニフォームを
//! 受け取り、3つそろったディスクリプタを返す」ことである。触れるのはディスクリプタのハンドルだけであり、
//! パイプラインは1本も作らない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::base_resources::大気LUT基盤資源;
use crate::vulkan::atmosphere_lut::multiscatter_descriptor::{多重散乱の束縛先, 多重散乱ディスクリプタ};
use crate::vulkan::atmosphere_lut::skyview_descriptor::{スカイビューの束縛先, スカイビューディスクリプタ};
use crate::vulkan::atmosphere_lut::transmittance_descriptor::透過率ディスクリプタ;

/// 3つそろったディスクリプタ。そろってから初めてパイプラインを作る。
pub(super) struct ディスクリプタ三点 {
    pub(super) 透過率: 透過率ディスクリプタ,
    pub(super) 多重散乱: 多重散乱ディスクリプタ,
    pub(super) スカイビュー: スカイビューディスクリプタ,
}

impl ディスクリプタ三点 {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.スカイビュー.破棄する(device);
        self.多重散乱.破棄する(device);
        self.透過率.破棄する(device);
    }
}

pub(super) fn 作る(device: &ash::Device, 基盤: &大気LUT基盤資源) -> Result<ディスクリプタ三点, レンダラーエラー> {
    let ユニフォーム一覧 = 基盤.ユニフォーム一覧();
    let 透過率 = 透過率ディスクリプタ::生成する(device, ユニフォーム一覧, 基盤.透過率.画像ビュー)?;
    let 多重散乱 = match 多重散乱を作る(device, 基盤, &ユニフォーム一覧) {
        Ok(値) => 値,
        Err(誤り) => {
            透過率.破棄する(device);
            return Err(誤り);
        }
    };
    match スカイビューを作る(device, 基盤, &ユニフォーム一覧) {
        Ok(スカイビュー) => Ok(ディスクリプタ三点 {
            透過率,
            多重散乱,
            スカイビュー,
        }),
        Err(誤り) => {
            多重散乱.破棄する(device);
            透過率.破棄する(device);
            Err(誤り)
        }
    }
}

fn 多重散乱を作る(
    device: &ash::Device,
    基盤: &大気LUT基盤資源,
    ユニフォーム一覧: &[vk::Buffer; crate::vulkan::sync::フレームインフライト数],
) -> Result<多重散乱ディスクリプタ, レンダラーエラー> {
    多重散乱ディスクリプタ::生成する(
        device,
        多重散乱の束縛先 {
            ユニフォーム一覧,
            透過率ビュー: 基盤.透過率.画像ビュー,
            多重散乱ビュー: 基盤.多重散乱.画像ビュー,
        },
    )
}

fn スカイビューを作る(
    device: &ash::Device,
    基盤: &大気LUT基盤資源,
    ユニフォーム一覧: &[vk::Buffer; crate::vulkan::sync::フレームインフライト数],
) -> Result<スカイビューディスクリプタ, レンダラーエラー> {
    スカイビューディスクリプタ::生成する(
        device,
        スカイビューの束縛先 {
            ユニフォーム一覧,
            透過率ビュー: 基盤.透過率.画像ビュー,
            多重散乱ビュー: 基盤.多重散乱.画像ビュー,
            スカイビュービュー: 基盤.スカイビュー.画像ビュー,
        },
    )
}
