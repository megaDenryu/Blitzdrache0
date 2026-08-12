//! 共有資源を順に確保する局面。呼ばれるのはレンダラー生成時の1度だけである。
//! 途中で失敗したら、そこまでに確保した資源をその場で逆順に破棄するため、部分的に生成された組は呼び出し元から見えない。
//!
//! 材質テクスチャ表の容量と照明の束縛レイアウトを受け取るのは、どちらもセットレイアウトの一部だからである。
//! 表の要素数はディスクリプタ配列の要素数であり、照明の束縛レイアウトは照明問い合わせのセットが宣言するバインドの並びを決める。

use ash::vk;

use super::共有資源;
use crate::cascade::影の一辺解像度;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::descriptor::{シーンセットレイアウト一式, 共有ディスクリプタセット};
use crate::vulkan::lighting_query::照明問い合わせ資源束;
use crate::vulkan::material_table::テクスチャ表レイアウト容量;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;

pub(in crate::renderer::generate) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    queue: vk::Queue,
    queue_family_index: u32,
    表容量: テクスチャ表レイアウト容量,
    影の一辺: 影の一辺解像度,
    照明束縛: 照明束縛レイアウト,
) -> Result<共有資源, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 影の資源 = vulkan::shadow_resources::影の資源の組::確保する(確保係, 影の一辺)?;
    let 転送 = match vulkan::transfer::転送実行環境::生成する(device, queue, queue_family_index) {
        Ok(値) => 値,
        Err(誤り) => {
            影の資源.破棄する(device);
            return Err(誤り);
        }
    };
    // 立方体配列を生成直後に深度読み取りのレイアウトへ移す。影を落とす灯を1件も持たない世界はこの画像へ
    // 1本もパスを積まず、レンダーグラフがバリアを1つも出さないためである(未定義のレイアウトのまま束縛される)。
    if let Err(誤り) = 影の資源.点光源の影.初期レイアウトを深度読み取りへ整える(&転送) {
        転送.破棄する();
        影の資源.破棄する(device);
        return Err(誤り);
    }
    let シェーダー定数 = match vulkan::uniform::フレームシェーダー定数一式::生成する(確保係) {
        Ok(値) => 値,
        Err(誤り) => {
            転送.破棄する();
            影の資源.破棄する(device);
            return Err(誤り);
        }
    };
    let セットレイアウト = match シーンセットレイアウト一式::生成する(確保係, 表容量, 照明束縛) {
        Ok(値) => 値,
        Err(誤り) => {
            シェーダー定数.破棄する(device);
            転送.破棄する();
            影の資源.破棄する(device);
            return Err(誤り);
        }
    };
    let 共有結果 = 共有ディスクリプタセット::生成する(device, &セットレイアウト, &シェーダー定数);
    let 共有ディスクリプタ = match 共有結果 {
        Ok(値) => 値,
        Err(誤り) => {
            セットレイアウト.破棄する(device);
            シェーダー定数.破棄する(device);
            転送.破棄する();
            影の資源.破棄する(device);
            return Err(誤り);
        }
    };
    let 照明結果 = 照明問い合わせ資源束::生成する(確保係, &セットレイアウト, &影の資源);
    let 照明問い合わせ = match 照明結果 {
        Ok(値) => 値,
        Err(誤り) => {
            共有ディスクリプタ.破棄する(device);
            セットレイアウト.破棄する(device);
            シェーダー定数.破棄する(device);
            転送.破棄する();
            影の資源.破棄する(device);
            return Err(誤り);
        }
    };
    Ok(共有資源 {
        影の資源,
        転送,
        シェーダー定数,
        セットレイアウト,
        共有ディスクリプタ,
        照明問い合わせ,
    })
}
