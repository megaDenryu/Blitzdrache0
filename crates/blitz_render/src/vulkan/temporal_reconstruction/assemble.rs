//! 確保済みの画像組へ標本器・ディスクリプタ・パイプラインを足して一式を組み上げる工程。
//! 受け取るのはシェーダーと描画設定と画像組、返すのは組み上がった一式である。
//!
//! 独立した工程にするのは、途中で失敗したときに「そこまでに作った分だけ」を逆順で片付ける後始末が要るためである。
//! 生成の入口にこの後始末を置くと、入口が画像組の確保・初期値の書き込み・束縛という別の関心事と同居する。

use ash::vk;

use super::descriptor::時間再構成のディスクリプタ;
use super::history_state::履歴の状態;
use super::images::時間再構成の画像組;
use super::pipeline::時間再構成のパイプライン;
use super::時間再構成一式;
use crate::error::レンダラーエラー;
use crate::temporal_reconstruction::{時間再構成のシェーダー一式, 時間再構成の描画設定};
use crate::vulkan::linear_sampler;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 組み上げる(
    device: &GPUデバイス,
    シェーダー: &時間再構成のシェーダー一式,
    設定: 時間再構成の描画設定,
    画像組: 時間再構成の画像組,
) -> Result<時間再構成一式, レンダラーエラー> {
    let 標本器 = match linear_sampler::線形サンプラーを作る(device) {
        Ok(標本器) => 標本器,
        Err(誤り) => {
            画像組.破棄する(device);
            return Err(誤り);
        }
    };
    let ディスクリプタ = match 時間再構成のディスクリプタ::生成する(device) {
        Ok(ディスクリプタ) => ディスクリプタ,
        Err(誤り) => {
            標本器を片付ける(device, 標本器);
            画像組.破棄する(device);
            return Err(誤り);
        }
    };
    match 時間再構成のパイプライン::生成する(device, ディスクリプタ.レイアウト(), &シェーダー.再構成) {
        Ok(パイプライン) => Ok(時間再構成一式 {
            画像組,
            ディスクリプタ,
            パイプライン,
            標本器,
            履歴の状態: 履歴の状態::無効で生成する(),
            合成入力の注入: None,
            設定,
        }),
        Err(誤り) => {
            ディスクリプタ.破棄する(device);
            標本器を片付ける(device, 標本器);
            画像組.破棄する(device);
            Err(誤り)
        }
    }
}

fn 標本器を片付ける(device: &ash::Device, 標本器: vk::Sampler) {
    // 安全性: 標本器はこのスコープの唯一の所有者で、以降使用しない。
    unsafe { device.destroy_sampler(標本器, None) };
}
