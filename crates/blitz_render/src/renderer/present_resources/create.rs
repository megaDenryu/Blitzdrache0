//! 生成局面: 起動時に1度呼ぶ初回生成と、寸法変更を受けた次フレームに1度呼ぶ作り直し。
//! どちらもスワップチェーンを先に作り、それが実際に決めた寸法と画像数から深度バッファと提示同期を作る。
//! 要求寸法はサーフェスの許す範囲へ丸められうるため、要求寸法で深度画像を作ると寸法がずれる。
//! 3つの整合を決める箇所をこの1つの手順に閉じることで、初回と作り直しで違う寸法の組み合わせが生まれない。
//!
//! 前提: 作り直しはGPU上の全作業の完了待ちを呼び出し元が済ませてから呼ぶ(旧資源の破棄を含むため)。

use super::提示資源;
use crate::error::レンダラーエラー;
use crate::extent::{ウィンドウ寸法, 非ゼロ寸法};
use crate::vulkan;
use crate::vulkan::gpu_environment::GPU環境;

impl 提示資源 {
    pub(in crate::renderer) fn 生成する(環境: &GPU環境, 要求寸法: ウィンドウ寸法) -> Result<Self, レンダラーエラー> {
        let device = 環境.device();
        let swapchain = 環境.スワップチェーンを作る(要求寸法, None)?;
        let メモリプロパティ = 環境.メモリプロパティを取得する();
        let 深度バッファ = match vulkan::depth::深度バッファ::生成する(device, &メモリプロパティ, swapchain.寸法) {
            Ok(値) => 値,
            Err(誤り) => {
                swapchain.破棄する(device, 環境.swapchain_loader());
                return Err(誤り);
            }
        };
        let 提示同期 = match vulkan::sync::提示同期::生成する(device, swapchain.画像数()) {
            Ok(値) => 値,
            Err(誤り) => {
                深度バッファ.破棄する(device);
                swapchain.破棄する(device, 環境.swapchain_loader());
                return Err(誤り);
            }
        };
        Ok(Self {
            swapchain,
            深度バッファ,
            提示同期,
        })
    }

    /// 旧資源を破棄してから作り直す。Vulkanのold_swapchain経路は使わない(破棄済みハンドルを渡せない)。
    ///
    /// 注意: 破棄を先に行うため、生成の失敗はこの束を使用不能な状態のまま残す。呼び出し元は
    /// このエラーをレンダラー全体の続行不能として扱う(blitz_appはフレームループを終了させる)。
    pub(in crate::renderer) fn 作り直す(&mut self, 環境: &GPU環境, 要求寸法: 非ゼロ寸法) -> Result<(), レンダラーエラー> {
        self.破棄する(環境);
        *self = Self::生成する(環境, 要求寸法.ウィンドウ寸法へ())?;
        Ok(())
    }
}
