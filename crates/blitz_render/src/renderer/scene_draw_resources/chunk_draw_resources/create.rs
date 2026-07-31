//! 束1つぶんのGPU資源を確保する局面。呼ばれるのは束の読込時の1度だけであり、毎フレームの走査とは呼び出し頻度が異なる。
//! 途中で失敗したときに確保済みの資源を逆順で解放する規律をここが持つため、毎フレームの読み取り側にはその分岐が現れない。

use super::super::create::チャンク描画資源生成材料;
use super::super::render_object_resources::{self, 描画対象資源};
use super::{チャンク描画資源, 資源一覧を破棄する};
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan::descriptor::描画対象ディスクリプタプール;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::可視ID列バッファ;

impl チャンク描画資源 {
    /// 失敗したときは生成途中のGPU資源をすべて解放してからエラーを返すため、呼び出し元は自分が保持中の束をそのまま使い続けられる。
    pub(in crate::renderer::scene_draw_resources) fn 生成する(
        device: &GPUデバイス,
        材料: チャンク描画資源生成材料<'_>,
        id: 描画束ID,
        描画対象一覧: &[描画対象素材],
    ) -> Result<Self, レンダラーエラー> {
        let 描画対象資源一覧 = render_object_resources::描画対象資源一覧を生成する(
            材料.物理デバイス問い合わせ,
            device,
            材料.メモリプロパティ,
            材料.転送環境,
            描画対象一覧,
        )?;
        let 単一個体用可視id列 = match 可視ID列バッファ::生成する(device, 材料.メモリプロパティ, 1) {
            Ok(値) => 値,
            Err(誤り) => {
                資源一覧を破棄する(device, &描画対象資源一覧);
                return Err(誤り);
            }
        };
        let ディスクリプタ = match ディスクリプタを生成する(device, 材料, &描画対象資源一覧, &単一個体用可視id列) {
            Ok(値) => 値,
            Err(誤り) => {
                単一個体用可視id列.破棄する(device);
                資源一覧を破棄する(device, &描画対象資源一覧);
                return Err(誤り);
            }
        };
        Ok(Self {
            id,
            描画対象資源一覧,
            単一個体用可視id列,
            ディスクリプタ,
        })
    }
}

fn ディスクリプタを生成する(
    device: &GPUデバイス,
    材料: チャンク描画資源生成材料<'_>,
    描画対象資源一覧: &[描画対象資源],
    単一個体用可視id列: &可視ID列バッファ,
) -> Result<描画対象ディスクリプタプール, レンダラーエラー> {
    let 参照一覧 = 描画対象資源一覧
        .iter()
        .map(|資源| 資源.ディスクリプタ参照(単一個体用可視id列))
        .collect::<Vec<_>>();
    描画対象ディスクリプタプール::生成する(device, 材料.レイアウト, &参照一覧, 材料.シェーダー定数, 材料.シャドウマップ)
}
