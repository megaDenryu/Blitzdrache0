//! 描画対象が読込時に一度だけ書く2本のレコード列(個体変換と材質レコード)の確保と解放。担当するのは、
//! どちらも束の読込時に書いて以後変えない静的なストレージバッファであることと、片方の確保が失敗したときに
//! もう一方を取り残さないことである。描画はどちらも添字で1件を選ぶ。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_object_material::{描画対象素材, 材質スロット素材一覧};
use crate::vulkan::instance_transform::content::個体変換内容;
use crate::vulkan::instance_transform::個体変換バッファ;
use crate::vulkan::material_record::content::材質レコード内容;
use crate::vulkan::material_record::材質レコードバッファ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(super) struct レコード列一式 {
    pub(super) 個体変換: 個体変換バッファ,
    pub(super) 材質レコード: 材質レコードバッファ,
}

impl レコード列一式 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        素材: &描画対象素材,
    ) -> Result<Self, レンダラーエラー> {
        let 個体変換内容一覧 = 個体変換内容::一覧を作る(素材.個体変換一覧())?;
        let 個体変換 = 個体変換バッファ::生成する(device, メモリプロパティ, 転送環境, &個体変換内容一覧)?;
        let 材質レコード内容一覧 = 材質レコード列を作る(素材.材質スロット素材一覧());
        match 材質レコードバッファ::生成する(device, メモリプロパティ, 転送環境, &材質レコード内容一覧) {
            Ok(材質レコード) => Ok(Self {
                個体変換, 材質レコード
            }),
            Err(誤り) => {
                個体変換.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.材質レコード.破棄する(device);
        self.個体変換.破棄する(device);
    }
}

/// 材質スロットの並び順にレコードを作る。この順序が束内材質参照の添字の意味そのものであり、
/// スロット別のテクスチャの並びと同じ順でなければならない。
fn 材質レコード列を作る(素材一覧: &材質スロット素材一覧) -> Vec<材質レコード内容> {
    素材一覧
        .一覧()
        .iter()
        .map(|素材| 材質レコード内容::マテリアルから作る(素材.マテリアル()))
        .collect()
}
