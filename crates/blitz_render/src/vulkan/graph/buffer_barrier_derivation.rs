//! バッファ版バリア導出の純粋部分。画像版(`barrier_derivation`)と対になるが、
//! バッファにはレイアウトが無く「グラフ終端」への最終遷移も無い(参照:
//! `_doc/設計/レンダーグラフ.md`「仮想リソース」のグラフ管理バッファは次フレームの
//! 冒頭で初期状態からの差分として自然に同期される)ため、パスごとの結果のみを返す。

#[cfg(test)]
mod buffer_barrier_derivation_tests;

use std::collections::HashMap;

use super::handle::バッファハンドル;
use super::pass_resource_usage::パスリソース使用;
use super::usage::buffer_usage_mapping::{書き込みを含むか, 状態へ写像する};
use super::usage::バッファ用途;
use crate::vulkan::graph::buffer_state::バッファ状態;

/// 1つのバッファハンドルに対する遷移(前状態→今状態)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct バッファバリア記述 {
    pub(crate) ハンドル: バッファハンドル,
    pub(crate) 前: バッファ状態,
    pub(crate) 今: バッファ状態,
}

/// パス列を宣言順に走査し、パスごとに発行すべきバッファバリア記述を求める。
/// 戻り値は`パス列`と同じ長さ(添字が1対1対応)。
pub(crate) fn 導出する(
    初期状態: &HashMap<バッファハンドル, バッファ状態>, パス列: &[パスリソース使用]
) -> Vec<Vec<バッファバリア記述>> {
    let mut 現在状態 = 初期状態.clone();
    パス列
        .iter()
        .map(|パス| {
            パス
                .読みバッファ
                .iter()
                .chain(パス.書きバッファ.iter())
                .filter_map(|&(ハンドル, 用途)| 差分を計算する(&mut 現在状態, ハンドル, 用途))
                .collect()
        })
        .collect()
}

/// 1リソースぶんの差分を求め、状態表を更新する。バリアが不要なら`None`を返す。
///
/// 省略規則: 前後どちらのアクセスにも書き込みが含まれない(読み→読み)場合のみ省略する
/// (バッファにレイアウトが無いため、画像版と異なりレイアウト一致の判定は不要)。
fn 差分を計算する(
    現在状態: &mut HashMap<バッファハンドル, バッファ状態>,
    ハンドル: バッファハンドル,
    今用途: バッファ用途,
) -> Option<バッファバリア記述> {
    let 今状態 = 状態へ写像する(今用途);
    let 前状態 = *現在状態
        .get(&ハンドル)
        .unwrap_or_else(|| panic!("グラフに未登録のバッファハンドルがパスで使用された"));
    現在状態.insert(ハンドル, 今状態);

    let 省略可能 = !書き込みを含むか(前状態.access) && !書き込みを含むか(今状態.access);
    if 省略可能 {
        return None;
    }
    Some(バッファバリア記述 {
        ハンドル,
        前: 前状態,
        今: 今状態,
    })
}
