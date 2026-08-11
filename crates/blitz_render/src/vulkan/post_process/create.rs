//! ポスト処理一式の生成手順(判断38・39・41)。途中失敗時は生成済みの資源を逆順で片付ける。
//! 寸法に連動する2枚の画像の生成は`sized_images`が持つ(`resize`も同じ工程を呼ぶ)。

use ash::vk;

use super::ポスト処理一式;
use crate::auto_exposure::自動露出の設定;
use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::auto_exposure::自動露出一式;
use crate::vulkan::bloom::光のにじみ一式;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tonemap::明るさの圧縮一式;
use crate::vulkan::transfer::転送実行環境;

/// 生成に要る材料一式。
pub(super) struct 生成材料<'a> {
    pub(super) 確保係: &'a GPU資源の確保係<'a>,
    pub(super) スワップチェーン画像形式: vk::Format,
    pub(super) 寸法: vk::Extent2D,
    pub(super) シェーダー: &'a シェーダー束,
    pub(super) 転送環境: &'a 転送実行環境,
    pub(super) 自動露出の設定: 自動露出の設定,
}

pub(super) fn 生成する(材料: 生成材料<'_>) -> Result<ポスト処理一式, レンダラーエラー> {
    let device = 材料.確保係.論理デバイス();
    let (hdrターゲット, 光のにじみピラミッド) = super::sized_images::生成する(材料.確保係, 材料.寸法)?;
    match 束を生成する(&材料, &hdrターゲット, &光のにじみピラミッド) {
        Ok((光のにじみ, 明るさの圧縮, 自動露出)) => Ok(ポスト処理一式 {
            hdrターゲット,
            光のにじみピラミッド,
            光のにじみ,
            明るさの圧縮,
            自動露出,
        }),
        Err(誤り) => {
            光のにじみピラミッド.破棄する(device);
            hdrターゲット.破棄する(device);
            Err(誤り)
        }
    }
}

/// 寸法に依存しないパイプラインとサンプラーの部分。生成直後の画像ビュー束縛も各一式の生成が行う。
/// 自動露出を最後に作るのは、明るさの圧縮のディスクリプタが露出状態のバッファを束縛するためである。
fn 束を生成する(
    材料: &生成材料<'_>,
    hdr: &HDRターゲット,
    ピラミッド: &光のにじみピラミッド,
) -> Result<(光のにじみ一式, 明るさの圧縮一式, 自動露出一式), レンダラーエラー> {
    let device = 材料.確保係.論理デバイス();
    let 光のにじみ = 光のにじみ一式::生成する(
        材料.確保係,
        &材料.シェーダー.光のにじみ前処理,
        &材料.シェーダー.光のにじみ縮小,
        &材料.シェーダー.光のにじみ拡大,
        hdr.画像ビュー,
        ピラミッド,
    )?;
    let 圧縮結果 = 明るさの圧縮一式::生成する(
        材料.確保係,
        材料.スワップチェーン画像形式,
        &材料.シェーダー.明るさの圧縮,
        hdr.画像ビュー,
        ピラミッド.最終ビュー(),
    );
    let 明るさの圧縮 = match 圧縮結果 {
        Ok(一式) => 一式,
        Err(誤り) => {
            光のにじみ.破棄する(device);
            return Err(誤り);
        }
    };
    let 自動露出結果 = 自動露出一式::生成する(材料.確保係, 材料.転送環境, &材料.シェーダー.自動露出, 材料.自動露出の設定, hdr.画像ビュー);
    match 自動露出結果 {
        Ok(一式) => {
            明るさの圧縮.露出状態を束縛する(device, 一式.露出状態バッファ());
            Ok((光のにじみ, 明るさの圧縮, 一式))
        }
        Err(誤り) => {
            明るさの圧縮.破棄する(device);
            光のにじみ.破棄する(device);
            Err(誤り)
        }
    }
}
