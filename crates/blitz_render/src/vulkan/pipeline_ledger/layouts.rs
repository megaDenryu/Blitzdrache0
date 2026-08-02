//! 材質描画族(シーン・シャドウ)のパイプラインレイアウトの所有者。担当するのは、族ごとに1つだけレイアウトを持ち、
//! その族の全パイプラインへ同じものを配ることである。
//!
//! 族ごとに1つにする理由は2つある。1つは所有をはっきりさせることであり、パイプラインごとにレイアウトを持たせると
//! 同じ宣言のレイアウトが記載の数だけできて、破棄も記載の数だけ要る(どれか1つが二重に破棄すると未定義動作になる)。
//! もう1つは、同じ族のパイプラインを切り替えてもディスクリプタセットの束縛が無効にならないことを構造で保証するためである。
//! Vulkanのセットはパイプラインレイアウトの互換性で生死が決まるため、族の中でレイアウトが1つなら、パスの先頭で1回結んだ
//! set0・set2・set3はパイプラインを切り替えても生き続ける
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::シーンセットレイアウト一式;
use crate::vulkan::{pipeline, scene_draw_constants, shadow_push};

pub(crate) struct 材質描画族のレイアウト {
    シーン: vk::PipelineLayout,
    シャドウ: vk::PipelineLayout,
}

impl 材質描画族のレイアウト {
    /// 呼ばれるのはレンダラー生成時の1回だけである。シャドウの生成に失敗したらシーンのぶんをその場で破棄する。
    pub(super) fn 生成する(device: &ash::Device, セット: &シーンセットレイアウト一式) -> Result<Self, レンダラーエラー> {
        let シーン = pipeline::レイアウトを生成する(device, &セット.シーンの並び(), scene_draw_constants::プッシュ定数範囲())?;
        let シャドウ = match pipeline::レイアウトを生成する(device, &セット.シャドウの並び(), shadow_push::プッシュ定数範囲())
        {
            Ok(レイアウト) => レイアウト,
            Err(誤り) => {
                pipeline::レイアウトを破棄する(device, シーン);
                return Err(誤り);
            }
        };
        Ok(Self { シーン, シャドウ })
    }

    pub(crate) const fn シーン(&self) -> vk::PipelineLayout {
        self.シーン
    }

    pub(crate) const fn シャドウ(&self) -> vk::PipelineLayout {
        self.シャドウ
    }

    /// 前提: このレイアウトで作った全パイプラインを破棄した後に呼ぶ。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        pipeline::レイアウトを破棄する(device, self.シャドウ);
        pipeline::レイアウトを破棄する(device, self.シーン);
    }
}
