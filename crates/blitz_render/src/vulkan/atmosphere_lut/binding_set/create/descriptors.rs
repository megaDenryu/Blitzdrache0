//! 4つのディスクリプタの生成と、途中で失敗したときの巻き戻し。担当する工程は「基盤資源のビューとユニフォームを
//! 受け取り、4つそろったディスクリプタを返す」ことである。触れるのはディスクリプタのハンドルだけであり、
//! パイプラインは1本も作らない。
//!
//! スカイビューと空中遠近は同じ型であり書き込み先だけが違うため、2つの生成は`march_pair`が並びで扱う。

mod march_pair;

use ash::vk;

use march_pair::経路生成を順に作る;

use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::base_resources::大気のベイク済み画像の基盤資源;
use crate::vulkan::atmosphere_lut::march_descriptor::経路生成ディスクリプタ;
use crate::vulkan::atmosphere_lut::multiscatter_descriptor::{多重散乱の束縛先, 多重散乱ディスクリプタ};
use crate::vulkan::atmosphere_lut::transmittance_descriptor::透過率ディスクリプタ;
use crate::vulkan::sync::フレームインフライト数;

/// 4つそろったディスクリプタ。そろってから初めてパイプラインを作る。
pub(super) struct ディスクリプタ四点 {
    pub(super) 透過率: 透過率ディスクリプタ,
    pub(super) 多重散乱: 多重散乱ディスクリプタ,
    pub(super) スカイビュー: 経路生成ディスクリプタ,
    pub(super) 空中遠近: 経路生成ディスクリプタ,
}

impl ディスクリプタ四点 {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        self.空中遠近.破棄する(device);
        self.スカイビュー.破棄する(device);
        self.多重散乱.破棄する(device);
        self.透過率.破棄する(device);
    }
}

pub(super) fn 作る(
    device: &ash::Device, 基盤: &大気のベイク済み画像の基盤資源
) -> Result<ディスクリプタ四点, レンダラーエラー> {
    let ユニフォーム一覧 = 基盤.ユニフォーム一覧();
    let 透過率 = 透過率ディスクリプタ::生成する(device, ユニフォーム一覧, 基盤.透過率.画像ビュー)?;
    let 多重散乱 = match 多重散乱を作る(device, 基盤, &ユニフォーム一覧) {
        Ok(値) => 値,
        Err(誤り) => {
            透過率.破棄する(device);
            return Err(誤り);
        }
    };
    let 書き込み先一覧 = [基盤.スカイビュー.画像ビュー, 基盤.空中遠近.画像ビュー];
    match 経路生成を順に作る(device, 基盤, &ユニフォーム一覧, 書き込み先一覧) {
        Ok([スカイビュー, 空中遠近]) => Ok(ディスクリプタ四点 {
            透過率,
            多重散乱,
            スカイビュー,
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
    device: &ash::Device,
    基盤: &大気のベイク済み画像の基盤資源,
    ユニフォーム一覧: &[vk::Buffer; フレームインフライト数],
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
