//! ウィンドウを持たないGPUで大気LUTを焼き、中身をホストへ読み戻す工程。
//! 受け取るのは媒体・解像度・観測条件・シェーダー、返すのは3枚のLUTの生の4成分とvalidationの観測である。
//!
//! 値オブジェクトでなく生の4成分で返すのは、検査が非有限値や負の値を観測できなければならないためである。
//! 検証済みの型で返すと、まさに検査したい壊れた値が構築の時点で弾かれてしまう。
//!
//! 本番のフレーム記録と同じパス宣言(`pass`)を同じレンダーグラフへ積むため、バリアの導出も本番と同じ経路を通る。

mod readback_set;
mod record;
mod validation_observation;

use crate::atmosphere::{スカイビュー観測条件, 大気LUT解像度, 大気散乱媒体, 空中遠近観測条件};
use crate::atmosphere_lut_input::大気LUT生成指示;
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::headless::ヘッドレスGPU環境;
use crate::vulkan::sync::フレームスロット添字;

pub(crate) use validation_observation::検証観測;

use readback_set::受け皿一式;

use super::{大気LUT一式, 描画入力の材料};

/// 各LUTを焼いて読み戻した結果。並びは行優先であり、CPU正本が焼く並びと一致する。
pub(crate) struct 大気LUT読み戻し {
    pub(crate) 透過率: Vec<[f32; 4]>,
    pub(crate) 多重散乱: Vec<[f32; 4]>,
    pub(crate) スカイビュー: Vec<[f32; 4]>,
    pub(crate) 空中遠近: Vec<[f32; 4]>,
}

/// 焼く条件一式。引数の列が伸び続けるのを避けて1つに束ねる。
pub(crate) struct 焼く条件<'a> {
    pub(crate) 媒体: &'a 大気散乱媒体,
    pub(crate) 解像度: 大気LUT解像度,
    pub(crate) スカイビュー条件: スカイビュー観測条件,
    pub(crate) 空中遠近条件: 空中遠近観測条件,
    pub(crate) シェーダー: &'a 大気LUTシェーダー一式,
}

pub(crate) fn 大気lutをgpuで焼いて読み戻す(
    条件: 焼く条件<'_>
) -> Result<(大気LUT読み戻し, 検証観測), レンダラーエラー> {
    let 環境 = ヘッドレスGPU環境::生成する()?;
    let 結果 = 環境で焼く(&環境, &条件);
    let 観測 = 検証観測::環境を破棄して観測する(&環境);
    Ok((結果?, 観測))
}

fn 環境で焼く(環境: &ヘッドレスGPU環境, 条件: &焼く条件<'_>) -> Result<大気LUT読み戻し, レンダラーエラー> {
    let device = 環境.device();
    let メモリプロパティ = 環境.メモリプロパティを取得する();
    let 一式 = 大気LUT一式::生成する(device, &メモリプロパティ, 条件.解像度, 条件.シェーダー)?;
    let 結果 = 一式で焼く(環境, &一式, 条件, &メモリプロパティ);
    一式.破棄する(device);
    結果
}

fn 一式で焼く(
    環境: &ヘッドレスGPU環境,
    一式: &大気LUT一式,
    条件: &焼く条件<'_>,
    メモリプロパティ: &ash::vk::PhysicalDeviceMemoryProperties,
) -> Result<大気LUT読み戻し, レンダラーエラー> {
    let device = 環境.device();
    let スロット = フレームスロット添字::先頭();
    一式.媒体を書き込む(device, スロット, 条件.媒体)?;
    let 入力 = 一式.描画入力を作る(&描画入力の材料 {
        フレーム添字: スロット,
        媒体: 条件.媒体,
        スカイビュー条件: 条件.スカイビュー条件,
        空中遠近条件: 条件.空中遠近条件,
        指示: 大気LUT生成指示::全部焼き直す,
        空中遠近を引くか: true,
    });
    let 受け皿 = 受け皿一式::確保する(
        device,
        メモリプロパティ,
        [
            条件.解像度.透過率のテクセル数(),
            条件.解像度.多重散乱のテクセル数(),
            条件.解像度.スカイビューのテクセル数(),
            条件.解像度.空中遠近のボクセル数(),
        ],
    )?;
    let 結果 = record::焼いて読み戻す(環境, &入力, &受け皿);
    受け皿.破棄する(device);
    結果
}
