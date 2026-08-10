//! 描画段階資源の生成。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに生成した資源をその場で逆順に破棄する。生成の途中経過を外へ出さないため、
//! 部分的に生成された器は呼び出し元から見えない。
//! フレーム構成しだいで作る段階の資源は`optional_stages`が担う。シーンと影の本体のパイプラインはパイプライン台帳が持つため、ここでは作らない。

mod optional_stages;

use ash::vk;

use super::描画段階資源;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;
use crate::indirect_lighting::照明問い合わせ契約;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::cluster_light_assignment::クラスタ選別一式;
use crate::vulkan::descriptor::{シーンセットレイアウト一式, 照明問い合わせのバッファ組};
use crate::vulkan::tracked_device::GPUデバイス;

/// 器を組み立てるのに要る材料一式。
pub(in crate::renderer) struct 生成要求<'a> {
    pub(in crate::renderer) device: &'a GPUデバイス,
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    /// シーン段階の色アタッチメントの形式(ポスト処理があればHDR中間画像、無ければスワップチェーン)。
    /// 影段階は深度だけへ書くため色形式を要らない。
    pub(in crate::renderer) シーンカラー形式: vk::Format,
    pub(in crate::renderer) セットレイアウト: &'a シーンセットレイアウト一式,
    pub(in crate::renderer) シェーダー: &'a シェーダー束,
    pub(in crate::renderer) 構成: フレーム構成,
    /// 世界の間接照明方針から起動時に決まる契約。遠方環境の枝だけが遠方環境と派生表現の資源を作る。
    pub(in crate::renderer) 照明問い合わせ契約: 照明問い合わせ契約,
    /// 照明問い合わせ資源束が持つ、スロットごとのバッファの組。選別の生成側のセットがこれを結ぶ。
    pub(in crate::renderer) クラスタ選別が読むバッファ組一覧: &'a [照明問い合わせのバッファ組],
}

/// 選別を先に作るのは、この後に続く任意段階の資源に破棄をまとめて呼ぶ口が無く、後から作ると
/// 任意段階の失敗で選別だけが漏れる経路と、選別の失敗で任意段階が漏れる経路のどちらかが必ず残るためである。
pub(super) fn 生成する(要求: 生成要求<'_>) -> Result<描画段階資源, レンダラーエラー> {
    let クラスタ選別 = クラスタ選別一式::生成する(要求.device, &要求.シェーダー.クラスタ選別, 要求.クラスタ選別が読むバッファ組一覧)?;
    let 任意 = match optional_stages::組み立てる(&要求) {
        Ok(値) => 値,
        Err(誤り) => {
            クラスタ選別.破棄する(要求.device);
            return Err(誤り);
        }
    };
    Ok(描画段階資源 {
        空: 任意.空,
        大気のベイク済み画像: 任意.大気のベイク済み画像,
        遠方環境の照明: 任意.遠方環境の照明,
        布シャドウ: 任意.布シャドウ,
        合成深度の注入: None,
        クラスタ選別,
    })
}
