//! 画面と同じ寸法の画像を持つ2つの資源(局所可視性補正の可視度2枚と、時間再構成の動きベクトル1枚・履歴2枚)の組み立て。
//! 受け取るのは生成要求と基礎資源、返すのはその2つである。
//!
//! 1つの工程にまとめるのは、どちらも「画面寸法に連動し、スワップチェーンの作り直しで揃って作り直す、レンダラー所有の
//! 画像資源」という同じ扱いを受けるためである。片方だけを別の場所で作ると、寸法追従の呼び出しを足し忘れても
//! 起動は通り、リサイズしたときだけ古い寸法の画像を読む。
//! 材料に転送環境を取るため、基礎資源の組み立て後に呼ぶ。

use super::base_resources::基礎資源;
use super::request::生成要求;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::allocator::GPU資源の確保係;

/// 画面寸法に連動する資源の組。
pub(super) struct 画面連動資源 {
    pub(super) 局所可視性: vulkan::local_visibility::局所可視性一式,
    pub(super) 時間再構成: vulkan::temporal_reconstruction::時間再構成一式,
}

pub(super) fn 組み立てる(
    要求: &生成要求<'_>,
    確保係: &GPU資源の確保係<'_>,
    基礎: &基礎資源,
) -> Result<画面連動資源, レンダラーエラー> {
    let device = 要求.環境.device();
    let 局所可視性 = vulkan::local_visibility::局所可視性一式::生成する(
        確保係,
        &基礎.転送環境,
        &要求.シェーダー.局所可視性,
        要求.局所可視性の描画設定,
        vulkan::local_visibility::画面の入力 {
            寸法: 要求.提示.寸法(),
            深度ビュー: 要求.提示.深度ビュー(),
        },
    )?;
    match vulkan::temporal_reconstruction::時間再構成一式::生成する(
        確保係,
        &基礎.転送環境,
        &要求.シェーダー.時間再構成,
        要求.時間再構成の描画設定,
        vulkan::temporal_reconstruction::画面の入力 {
            寸法: 要求.提示.寸法(),
            深度ビュー: 要求.提示.深度ビュー(),
        },
    ) {
        Ok(時間再構成) => Ok(画面連動資源 {
            局所可視性, 時間再構成
        }),
        Err(誤り) => {
            局所可視性.破棄する(device);
            Err(誤り)
        }
    }
}
