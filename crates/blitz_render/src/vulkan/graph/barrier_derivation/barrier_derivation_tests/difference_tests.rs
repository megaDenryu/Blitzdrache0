//! 同一用途と空パスに対するバリア差分の省略規則を検証する。

use std::collections::HashMap;

use super::空のパス;
use crate::vulkan::graph::barrier_derivation::{バリアを導出する, 地点別バリア, 差分を計算する};
use crate::vulkan::graph::handle::画像ハンドル;
use crate::vulkan::graph::initial_state::取得直後の色画像状態;
use crate::vulkan::graph::usage::画像用途;

#[test]
fn 読みから読みでレイアウトが同じなら省略される() {
    let カラー = 画像ハンドル::生成する(0, 0);
    let mut 現在状態 = HashMap::new();
    現在状態.insert(カラー, 取得直後の色画像状態());

    // 転送元へ一度遷移させ、その後もう一度同じ転送元用途で使う(読み→読み・同一レイアウト)。
    let 一回目 = 差分を計算する(&mut 現在状態, カラー, 画像用途::転送元);
    assert!(一回目.is_some(), "初回はUNDEFINEDからの遷移なので発行される");

    let 二回目 = 差分を計算する(&mut 現在状態, カラー, 画像用途::転送元);
    assert!(二回目.is_none(), "読み→読みでレイアウトが同一なら省略するはず");
}

#[test]
fn 差分がない地点はバリア一覧が空になる() {
    let カラー = 画像ハンドル::生成する(0, 0);
    let mut 初期状態 = HashMap::new();
    初期状態.insert(カラー, 取得直後の色画像状態());

    let パス列 = vec![空のパス("何もしないパス")];
    let 結果: Vec<地点別バリア> = バリアを導出する(&初期状態, &パス列, &[]);

    assert_eq!(結果.len(), 2);
    assert!(結果[0].バリア一覧.is_empty());
    assert!(結果[1].バリア一覧.is_empty());
}
