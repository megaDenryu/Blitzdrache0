//! 時間再構成の資源一式。担当するのは、世界が宣言した時間再構成方式をGPU側で保ち、画面と同じ寸法の3枚
//! (動きベクトル1枚と履歴2枚)を確保して毎フレームのパスへ渡すハンドルを答えることである。
//!
//! 方式が使わないの世界でも3枚を確保するのは、確保するかどうかを方式で分けると、シーン描画のパイプラインが
//! 宣言するカラー添付の枚数が世界ごとに変わり、パイプラインの一致条件が1つ増えるためである。動きベクトルは
//! 方式に依らず毎フレーム書く。方式で分かれるのは、時間再構成のパスを積むかどうかと、履歴画像へ初期値を
//! 書き込むかどうかだけである(`参照: _doc/設計/時間再構成.md`「判断e: 世界宣言の第4軸と検収の保護」)。

mod fill;
mod images;
mod resize;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::temporal_reconstruction::時間再構成方式;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) use images::動きベクトルの形式;

pub(crate) struct 時間再構成一式 {
    画像組: images::時間再構成の画像組,
    方式: 時間再構成方式,
}

impl 時間再構成一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        方式: 時間再構成方式,
        寸法: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        let 画像組 = images::時間再構成の画像組::生成する(device, メモリプロパティ, 寸法)?;
        let 一式 = Self { 画像組, 方式 };
        if let Err(誤り) = 一式.履歴の初期値を書く(device, 転送環境, &一式.画像組) {
            一式.破棄する(device);
            return Err(誤り);
        }
        Ok(一式)
    }

    /// 履歴を読むパスが1本も積まれない方式では、確保した2枚の中身を誰も読まないため書き込まない。
    /// 対象の画像組を引数で受けるのは、寸法追従が古い組を持ったまま新しい組へ書くためである。
    pub(super) fn 履歴の初期値を書く(
        &self,
        device: &GPUデバイス,
        転送環境: &転送実行環境,
        画像組: &images::時間再構成の画像組,
    ) -> Result<(), レンダラーエラー> {
        if !self.方式.履歴画像を読むパスが積まれるか() {
            return Ok(());
        }
        fill::履歴を零で埋める(device, 転送環境, 画像組)
    }

    /// シーン描画と空のパスが第2のカラー添付として書く動きベクトル画像。方式に依らず毎フレーム書くため、
    /// 有無を`Option`で表さない。
    pub(crate) fn 動きベクトル画像組(&self) -> (vk::Image, vk::ImageView) {
        (self.画像組.動きベクトル.画像, self.画像組.動きベクトル.画像ビュー)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.画像組.破棄する(device);
    }
}
