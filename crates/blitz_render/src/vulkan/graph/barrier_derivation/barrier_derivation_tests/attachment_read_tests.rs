//! アタッチメントを読む用途で、直前の書き込みが読みへ可視化されることを検証する。
//!
//! 見るのはバリアの有無ではなくアクセスマスクの中身である。同じアタッチメント用途が続く区間は書き込みが絡むため
//! バリアそのものは必ず出るが、その依存が書き込みから書き込みへの向きしか持たないと、LOADによる読みと
//! 固定機能ブレンドによる宛先の読みは可視化の範囲に入らない。

use std::collections::HashMap;

use ash::vk;

use super::空のパス;
use crate::vulkan::graph::barrier_derivation::{導出する, 画像バリア記述};
use crate::vulkan::graph::handle::画像ハンドル;
use crate::vulkan::graph::initial_state::{前フレーム深度書き込み直後状態, 取得直後の色画像状態};
use crate::vulkan::graph::pass_resource_usage::パスリソース使用;
use crate::vulkan::graph::state::画像状態;
use crate::vulkan::graph::usage::画像用途;

/// 同じ画像を同じアタッチメント用途で使う2つのパスを並べ、2つめのパスの前に出るバリアを返す。
fn 続けて書く2パスの2つめの前のバリア(初期状態の値: 画像状態, 用途: 画像用途) -> 画像バリア記述 {
    let 画像 = 画像ハンドル::生成する(0, 0);
    let mut 初期状態 = HashMap::new();
    初期状態.insert(画像, 初期状態の値);
    let パス列 = vec![
        パスリソース使用 {
            書き画像: vec![(画像, 用途)],
            ..空のパス("先に書くパス")
        },
        パスリソース使用 {
            書き画像: vec![(画像, 用途)],
            ..空のパス("ロードしてから書くパス")
        },
    ];
    let 結果 = 導出する(&初期状態, &パス列, &[]);
    assert_eq!(結果[1].バリア一覧.len(), 1, "書き込みが絡むためバリアは省略されない");
    結果[1].バリア一覧[0]
}

/// 空中遠近合成は直前のシーンカラーをLOADし、ブレンドで宛先カラーも読む。この2種類の読みに対して、
/// 直前のカラー書き込みが可視化されていなければならない。
#[test]
fn カラー出力はロードとブレンドの読みを可視化する() {
    let バリア = 続けて書く2パスの2つめの前のバリア(取得直後の色画像状態(), 画像用途::カラー出力);
    assert!(
        バリア.前.access.contains(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE),
        "依存の元は直前のカラー書き込みである"
    );
    assert!(
        バリア.今.access.contains(vk::AccessFlags2::COLOR_ATTACHMENT_READ),
        "LOADとブレンドの読みが依存の先に入っていない"
    );
    assert!(
        バリア.今.access.contains(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE),
        "書き込みも依然として依存の先に入る"
    );
}

/// 深度も同じ理由で読みを持つ。深度テストが既存の値を比べ、LOADするパスはレンダリング開始そのものが読みである。
/// カラーと深度が同じ扱いであることを1箇所で固定する。
#[test]
fn 深度出力も深度テストとロードの読みを可視化する() {
    let バリア = 続けて書く2パスの2つめの前のバリア(前フレーム深度書き込み直後状態(), 画像用途::深度出力);
    assert!(バリア.今.access.contains(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ));
    assert!(バリア.今.access.contains(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE));
}
