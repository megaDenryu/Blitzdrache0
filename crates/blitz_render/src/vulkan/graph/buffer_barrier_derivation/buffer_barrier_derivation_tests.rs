//! バッファ版バリア導出の純粋部分の単体テスト。
//! 参照: `_doc/設計/レンダーグラフ.md`「同期導出の規則」。
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use super::{導出する, 差分を計算する};
use crate::vulkan::graph::handle::バッファハンドル;
use crate::vulkan::graph::initial_state::前フレーム粒子読み直後状態;
use crate::vulkan::graph::pass_resource_usage::パスリソース使用;
use crate::vulkan::graph::usage::バッファ用途;

fn 空のパス(名前: &'static str) -> パスリソース使用 {
    パスリソース使用 {
        名前,
        読み画像: Vec::new(),
        書き画像: Vec::new(),
        読みバッファ: Vec::new(),
        書きバッファ: Vec::new(),
    }
}

/// コンピュートで書いてから同じフレームの中で読む2本のパスを導出に掛ける。読む段だけが違う2つの検査が
/// 同じ形の列を要るため、組み立てをここへ寄せる。
fn コンピュートで書いてから読む(読み用途: バッファ用途) -> Vec<Vec<super::バッファバリア記述>> {
    let バッファ = バッファハンドル::生成する(0, 0);
    let mut 初期状態 = HashMap::new();
    初期状態.insert(バッファ, 前フレーム粒子読み直後状態());
    let パス列 = vec![
        パスリソース使用 {
            書きバッファ: vec![(バッファ, バッファ用途::コンピュート書き)],
            ..空のパス("コンピュートの書き込み")
        },
        パスリソース使用 {
            読みバッファ: vec![(バッファ, 読み用途)],
            ..空のパス("読み")
        },
    ];
    導出する(&初期状態, &パス列)
}

#[test]
fn コンピュート書きから頂点段シェーダー読みへの遷移でバリアが出る() {
    let 結果 = コンピュートで書いてから読む(バッファ用途::頂点段シェーダー読み);

    assert_eq!(結果.len(), 2, "パスごとに1エントリ(グラフ終端は無い)になるはず");
    assert_eq!(結果[0].len(), 1, "初回は前フレーム読み直後→コンピュート書きで遷移するはず");
    assert_eq!(結果[0][0].今.access, ash::vk::AccessFlags2::SHADER_STORAGE_WRITE);
    assert_eq!(結果[0][0].今.stage, ash::vk::PipelineStageFlags2::COMPUTE_SHADER);

    assert_eq!(結果[1].len(), 1, "コンピュート書き→頂点段シェーダー読みで遷移するはず(書きが絡む)");
    assert_eq!(結果[1][0].今.access, ash::vk::AccessFlags2::SHADER_STORAGE_READ);
    assert_eq!(結果[1][0].今.stage, ash::vk::PipelineStageFlags2::VERTEX_SHADER);
}

/// クラスタの選別が書いた格子と光添字列を、同じフレームのシーン描画が画素段で読む形である。読み宣言を落としても
/// 検証層は競合を報告しないため、導出そのものをここで固定する。
#[test]
fn コンピュート書きから画素段シェーダー読みへの遷移でバリアが出る() {
    let 結果 = コンピュートで書いてから読む(バッファ用途::画素段シェーダー読み);

    assert_eq!(結果.len(), 2, "パスごとに1エントリ(グラフ終端は無い)になるはず");
    assert_eq!(結果[1].len(), 1, "コンピュート書き→画素段シェーダー読みで遷移するはず(書きが絡む)");
    assert_eq!(結果[1][0].前.stage, ash::vk::PipelineStageFlags2::COMPUTE_SHADER);
    assert_eq!(結果[1][0].前.access, ash::vk::AccessFlags2::SHADER_STORAGE_WRITE);
    assert_eq!(結果[1][0].今.access, ash::vk::AccessFlags2::SHADER_STORAGE_READ);
    assert_eq!(結果[1][0].今.stage, ash::vk::PipelineStageFlags2::FRAGMENT_SHADER);
}

#[test]
fn 読みから読みは省略される() {
    let 粒子 = バッファハンドル::生成する(0, 0);
    let mut 現在状態 = HashMap::new();
    現在状態.insert(粒子, 前フレーム粒子読み直後状態());

    // 頂点段シェーダー読み→頂点段シェーダー読み(読み→読み)は書き込みが絡まないため省略。
    let 一回目 = 差分を計算する(&mut 現在状態, 粒子, バッファ用途::頂点段シェーダー読み);
    assert!(一回目.is_none(), "初期状態も読みのため読み→読みで省略されるはず");
}

#[test]
fn 差分がない地点はバリア一覧が空になる() {
    let 粒子 = バッファハンドル::生成する(0, 0);
    let mut 初期状態 = HashMap::new();
    初期状態.insert(粒子, 前フレーム粒子読み直後状態());

    let パス列 = vec![空のパス("何もしないパス")];
    let 結果 = 導出する(&初期状態, &パス列);

    assert_eq!(結果.len(), 1);
    assert!(結果[0].is_empty());
}
