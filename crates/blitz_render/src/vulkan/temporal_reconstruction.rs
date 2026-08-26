//! 時間再構成の資源一式。担当するのは、世界が宣言した時間再構成方式をGPU側で保ち、画面と同じ寸法の4枚(今のフレームの色1枚と動きベクトル1枚と履歴2枚)とパスの実体を確保して毎フレームのパスへ渡す入力を答えることである。
//!
//! 方式が使わないの世界でも4枚を確保するのは、確保するかどうかを方式で分けると、シーン描画のパイプラインが宣言するカラー添付の枚数が世界ごとに変わり、パイプラインの一致条件が1つ増えるためである。動きベクトルは方式に依らず毎フレーム書く。方式で分かれるのは、時間再構成のパスを積むかどうかと、履歴画像へ初期値を書き込むかどうかだけである(`参照: _doc/設計/時間再構成.md`「判断e」)。
//!
//! `合成入力の注入`は検収が合成入力を据えた実行だけが持ち、本番の起動では転送パスを1本も積まない。

mod assemble;
mod bind;
mod descriptor;
mod fill;
mod history_state;
mod images;
mod injection;
mod input;
mod pipeline;
mod resize;
mod setting;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::temporal_reconstruction::{時間再構成のシェーダー一式, 時間再構成の描画設定};
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) use images::動きベクトルの形式;
pub(crate) use injection::{合成入力の書き戻し先, 合成入力の注入を作る, 合成入力の注入入力};
pub(crate) use input::時間再構成描画入力;

pub(crate) struct 時間再構成一式 {
    画像組: images::時間再構成の画像組,
    ディスクリプタ: descriptor::時間再構成のディスクリプタ,
    パイプライン: pipeline::時間再構成のパイプライン,
    標本器: vk::Sampler, // 履歴を写し戻した位置で参照するための線形補間の標本器
    履歴の状態: history_state::履歴の状態,
    合成入力の注入: Option<injection::合成入力の注入一式>, // 検収が合成入力を据えた実行だけ`Some`
    設定: 時間再構成の描画設定,
}

/// 生成のときと寸法へ追従したときで別々になりうる値の組。画面寸法と深度のビューは必ず同じ世代のものである。
#[derive(Clone, Copy)]
pub(crate) struct 画面の入力 {
    pub(crate) 寸法: vk::Extent2D,
    pub(crate) 深度ビュー: vk::ImageView,
}

impl 時間再構成一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &転送実行環境,
        シェーダー: &時間再構成のシェーダー一式,
        設定: 時間再構成の描画設定,
        画面: 画面の入力,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 画像組 = images::時間再構成の画像組::生成する(確保係, 画面.寸法)?;
        let 一式 = assemble::組み上げる(確保係, シェーダー, 設定, 画像組)?;
        if let Err(誤り) = 一式.履歴の初期値を書く(転送環境, &一式.画像組) {
            一式.破棄する(device);
            return Err(誤り);
        }
        一式.資源を束縛する(device, 画面.深度ビュー);
        Ok(一式)
    }

    /// 履歴を読むパスが1本も積まれない方式では、確保した2枚の中身を誰も読まないため書き込まない。対象の画像組を引数で受けるのは、寸法追従が古い組を持ったまま新しい組へ書くためである。
    pub(super) fn 履歴の初期値を書く(
        &self,
        転送環境: &転送実行環境,
        画像組: &images::時間再構成の画像組,
    ) -> Result<(), レンダラーエラー> {
        if !self.設定.方式.履歴画像を読むパスが積まれるか() {
            return Ok(());
        }
        fill::履歴を零で埋める(転送環境, 画像組)
    }

    /// シーン描画と空のパスが第2のカラー添付として書く動きベクトル画像。方式に依らず毎フレーム書くため、有無を`Option`で表さない。
    pub(crate) fn 動きベクトル画像組(&self) -> (vk::Image, vk::ImageView) {
        (self.画像組.動きベクトル.画像, self.画像組.動きベクトル.画像ビュー)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        if let Some(注入) = &self.合成入力の注入 {
            注入.破棄する(device);
        }
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        // 安全性: 標本器はSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_sampler(self.標本器, None) };
        self.画像組.破棄する(device);
    }
}
