//! 布の工程が刻み数ぶん積まれることの検証。グラフへ積むだけでGPUは1度も動かさないため、ハンドルは全て空でよい。

#![allow(clippy::unwrap_used)]

use ash::vk;

use super::{布ハンドル, 積む};
use crate::frame_input::布の進める刻み数;
use crate::vulkan::frame::{布シャドウ描画入力, 布描画の外部資源, 布描画入力};
use crate::vulkan::graph::{グラフ, 前フレームコンピュート読み直後状態, 前フレーム頂点入力読み直後状態};
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

fn 刻み数を据えた入力(刻み数: u32) -> 布描画入力 {
    布描画入力 {
        layout: vk::PipelineLayout::null(),
        介入pipeline: vk::Pipeline::null(),
        積分pipeline: vk::Pipeline::null(),
        アタッチpipeline: vk::Pipeline::null(),
        拘束pipeline: vk::Pipeline::null(),
        ハッシュ消去pipeline: vk::Pipeline::null(),
        ハッシュ格納pipeline: vk::Pipeline::null(),
        分離pipeline: vk::Pipeline::null(),
        仕上げpipeline: vk::Pipeline::null(),
        頂点生成pipeline: vk::Pipeline::null(),
        ディスクリプタセット: vk::DescriptorSet::null(),
        粒子数: 1024,
        アタッチ件数: 0,
        介入件数: 0,
        進める刻み数: 布の進める刻み数::生成する(刻み数).unwrap(),
        粒子バッファ: vk::Buffer::null(),
        前位置バッファ: vk::Buffer::null(),
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

fn 積んだパス名一覧(入力: &布描画入力) -> Vec<&'static str> {
    let mut グラフ = グラフ::新規();
    let ハンドル = 布ハンドル {
        布頂点: グラフ.バッファを登録する(入力.布頂点バッファ, 前フレーム頂点入力読み直後状態()),
        粒子: グラフ.バッファを登録する(入力.粒子バッファ, 前フレームコンピュート読み直後状態()),
        前位置: グラフ.バッファを登録する(入力.前位置バッファ, 前フレームコンピュート読み直後状態()),
        セルカウント: グラフ.バッファを登録する(入力.セルカウントバッファ, 前フレームコンピュート読み直後状態()),
        セル格納: グラフ.バッファを登録する(入力.セル格納バッファ, 前フレームコンピュート読み直後状態()),
    };
    積む(&mut グラフ, 入力, &ハンドル, None);
    let (_, _, _, _, _, パス列) = グラフ.分解する();
    パス列.into_iter().map(|パス| パス.名前).collect()
}

/// 反証: 刻み0本の描画でも工程を積むと、粒子が刻みを1本も進めない描画で進んでしまう。
#[test]
fn 刻み零本の描画はシミュレーションの工程を一本も積まない() {
    let 名前一覧 = 積んだパス名一覧(&刻み数を据えた入力(0));
    assert_eq!(名前一覧, vec!["布頂点生成"]);
}

/// 反証: 頂点生成を刻み数ぶん積むと、刻み0本の描画で布頂点バッファが未初期化のまま描かれる。
/// 頂点生成は時間を1つも進めないため、刻み数と無関係に描画機会ごとに1本だけ積む。
#[test]
fn 頂点生成は刻み数によらず描画機会ごとに一本だけ積まれる() {
    for 刻み数 in [0, 1, 2, 4] {
        let 名前一覧 = 積んだパス名一覧(&刻み数を据えた入力(刻み数));
        assert_eq!(名前一覧.iter().filter(|名前| **名前 == "布頂点生成").count(), 1);
    }
}

/// 反証: 描画ごとに1回だけ進めると、毎秒120回書き換えるモニターで布が2倍速、毎秒30回で半速になる。
/// 2本の描画は1本の描画のちょうど2倍の工程を積む。
#[test]
fn シミュレーションの工程は刻み数に比例して積まれる() {
    let 一本 = 積んだパス名一覧(&刻み数を据えた入力(1));
    let 二本 = 積んだパス名一覧(&刻み数を据えた入力(2));
    assert_eq!(一本.iter().filter(|名前| **名前 == "布積分").count(), 1);
    assert_eq!(二本.iter().filter(|名前| **名前 == "布積分").count(), 2);
    assert_eq!(二本.len(), 一本.len() * 2 - 1);
}

/// 反証: 上限を越えた本数を丸めて受け取ると、1フレームのパス数がクエリプールの容量を越えてpanicする。
#[test]
fn 上限を越えた刻み数は型付きの失敗になる() {
    let 上限 = 布の進める刻み数::上限の本数();
    assert!(布の進める刻み数::生成する(上限).is_ok());
    assert!(布の進める刻み数::生成する(上限 + 1).is_err());
}
