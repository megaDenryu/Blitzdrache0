//! 布の工程の検査が共有する材料: 空のハンドルで組んだ布の描画入力と、グラフへ積んだパス名の一覧。
//! グラフへ積むだけでGPUは1度も動かさないため、ハンドルは全て空でよい。

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use ash::vk;

use super::{布ハンドル, 積む};
use crate::cloth_material::{布の彩色の区間, 布の自己衝突};
use crate::frame_input::布の進める刻み数;
use crate::vulkan::frame::{布シャドウ描画入力, 布描画の外部資源, 布描画入力};
use crate::vulkan::graph::{グラフ, 前フレームコンピュート読み直後状態, 前フレーム頂点入力読み直後状態};
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

pub(super) const 検査の色の数: usize = 3;
pub(super) const 検査の曲げの色の数: usize = 2;
const 検査の目標拘束の数: u32 = 5;

pub(super) fn 刻み数を据えた入力(刻み数: u32) -> 布描画入力 {
    自己衝突を選んで刻み数を据えた入力(刻み数, 布の自己衝突::行う)
}

pub(super) fn 自己衝突を選んで刻み数を据えた入力(刻み数: u32, 自己衝突: 布の自己衝突) -> 布描画入力 {
    let mut 入力 = 目標拘束の数を据えた入力(刻み数, 検査の目標拘束の数);
    入力.自己衝突 = 自己衝突;
    入力
}

pub(super) fn 目標拘束の数を据えた入力(刻み数: u32, 目標拘束の数: u32) -> 布描画入力 {
    布描画入力 {
        layout: vk::PipelineLayout::null(),
        介入pipeline: vk::Pipeline::null(),
        積分pipeline: vk::Pipeline::null(),
        目標の確定pipeline: vk::Pipeline::null(),
        乗数零化pipeline: vk::Pipeline::null(),
        拘束pipeline: vk::Pipeline::null(),
        曲げ拘束pipeline: vk::Pipeline::null(),
        目標拘束pipeline: vk::Pipeline::null(),
        ハッシュ消去pipeline: vk::Pipeline::null(),
        ハッシュ格納pipeline: vk::Pipeline::null(),
        分離pipeline: vk::Pipeline::null(),
        床とカプセルの押し出しpipeline: vk::Pipeline::null(),
        仕上げpipeline: vk::Pipeline::null(),
        頂点生成pipeline: vk::Pipeline::null(),
        ディスクリプタセット: vk::DescriptorSet::null(),
        粒子数: 1024,
        拘束の数: 30,
        目標拘束の数,
        曲げ拘束の数: 8,
        色の区間一覧: vec![
            布の彩色の区間 { 開始: 0, 本数: 10 },
            布の彩色の区間 { 開始: 10, 本数: 10 },
            布の彩色の区間 { 開始: 20, 本数: 10 },
        ],
        曲げの色の区間一覧: vec![布の彩色の区間 { 開始: 0, 本数: 5 }, 布の彩色の区間 { 開始: 5, 本数: 3 }],
        自己衝突: 布の自己衝突::行う,
        目標の更新対応の件数: 0,
        介入件数: 0,
        進める刻み数: 布の進める刻み数::生成する(刻み数).unwrap(),
        粒子バッファ: vk::Buffer::null(),
        前位置バッファ: vk::Buffer::null(),
        拘束の引数バッファ: vk::Buffer::null(),
        ラグランジュ乗数バッファ: vk::Buffer::null(),
        目標拘束の引数バッファ: vk::Buffer::null(),
        目標位置バッファ: vk::Buffer::null(),
        目標の更新対応バッファ: vk::Buffer::null(),
        曲げ拘束の引数バッファ: vk::Buffer::null(),
        セルカウントバッファ: vk::Buffer::null(),
        セル格納バッファ: vk::Buffer::null(),
        布頂点バッファ: vk::Buffer::null(),
        インデックスバッファ: vk::Buffer::null(),
        インデックス数: 6,
        描画pipeline: vk::Pipeline::null(),
        描画layout: vk::PipelineLayout::null(),
        相対の基準原点: カメラ相対の基準原点::世界原点から生成する(blitz_math::大域ワールド位置::原点()).unwrap(),
        外部資源: 布描画の外部資源 {
            シャドウ: 布シャドウ描画入力 {
                pipeline: vk::Pipeline::null(),
                layout: vk::PipelineLayout::null(),
            },
        },
    }
}

pub(super) fn 積んだパス名一覧(入力: &布描画入力) -> Vec<&'static str> {
    let mut グラフ = グラフ::新規();
    let ハンドル = 布ハンドル {
        布頂点: グラフ.バッファを登録する(入力.布頂点バッファ, 前フレーム頂点入力読み直後状態()),
        粒子: グラフ.バッファを登録する(入力.粒子バッファ, 前フレームコンピュート読み直後状態()),
        前位置: グラフ.バッファを登録する(入力.前位置バッファ, 前フレームコンピュート読み直後状態()),
        拘束の引数: グラフ.バッファを登録する(入力.拘束の引数バッファ, 前フレームコンピュート読み直後状態()),
        ラグランジュ乗数: グラフ.バッファを登録する(入力.ラグランジュ乗数バッファ, 前フレームコンピュート読み直後状態()),
        目標拘束の引数: グラフ.バッファを登録する(入力.目標拘束の引数バッファ, 前フレームコンピュート読み直後状態()),
        目標位置: グラフ.バッファを登録する(入力.目標位置バッファ, 前フレームコンピュート読み直後状態()),
        目標の更新対応: グラフ.バッファを登録する(入力.目標の更新対応バッファ, 前フレームコンピュート読み直後状態()),
        曲げ拘束の引数: グラフ.バッファを登録する(入力.曲げ拘束の引数バッファ, 前フレームコンピュート読み直後状態()),
        セルカウント: グラフ.バッファを登録する(入力.セルカウントバッファ, 前フレームコンピュート読み直後状態()),
        セル格納: グラフ.バッファを登録する(入力.セル格納バッファ, 前フレームコンピュート読み直後状態()),
    };
    積む(&mut グラフ, 入力, &ハンドル, None);
    let (_, _, _, _, _, パス列) = グラフ.分解する();
    パス列.into_iter().map(|パス| パス.名前).collect()
}
