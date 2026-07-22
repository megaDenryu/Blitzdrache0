//! バリア導出の純粋部分: 初期状態とパス列・最終用途から、発行すべきバリア記述の列を
//! 副作用なしに求める。実行器はこの結果をそのままVulkan呼び出しへ変換するだけにする。
//! 参照: `_doc/設計/レンダーグラフ.md`「同期導出の規則」。

#[cfg(test)]
mod barrier_derivation_tests;

use std::collections::HashMap;

use super::handle::画像ハンドル;
use super::pass_resource_usage::パスリソース使用;
use super::state::画像状態;
use super::usage::image_usage_mapping::{書き込みを含むか, 状態へ写像する};
use super::usage::画像用途;

/// 1つの画像ハンドルに対する遷移(前状態→今状態)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 画像バリア記述 {
    pub(crate) ハンドル: 画像ハンドル,
    pub(crate) 前: 画像状態,
    pub(crate) 今: 画像状態,
}

/// バリアの発行地点: あるパスの直前、またはグラフ終端(最終用途への遷移)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 遷移地点 {
    パスの前 { 名前: &'static str },
    グラフ終端,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct 地点別バリア {
    pub(crate) 地点: 遷移地点,
    pub(crate) バリア一覧: Vec<画像バリア記述>,
}

/// パス列を宣言順に走査し、各地点で発行すべきバリア記述を求める。
pub(crate) fn 導出する(
    初期状態: &HashMap<画像ハンドル, 画像状態>,
    パス列: &[パスリソース使用],
    最終用途: &[(画像ハンドル, 画像用途)],
) -> Vec<地点別バリア> {
    let mut 現在状態 = 初期状態.clone();
    let mut 結果 = Vec::with_capacity(パス列.len() + 1);

    for パス in パス列 {
        let バリア一覧 = パス
            .読み画像
            .iter()
            .chain(パス.書き画像.iter())
            .filter_map(|&(ハンドル, 用途)| 差分を計算する(&mut 現在状態, ハンドル, 用途))
            .collect();
        結果.push(地点別バリア {
            地点: 遷移地点::パスの前 { 名前: パス.名前 },
            バリア一覧,
        });
    }

    let 終端バリア一覧 = 最終用途
        .iter()
        .filter_map(|&(ハンドル, 用途)| 差分を計算する(&mut 現在状態, ハンドル, 用途))
        .collect();
    結果.push(地点別バリア {
        地点: 遷移地点::グラフ終端,
        バリア一覧: 終端バリア一覧,
    });

    結果
}

/// 1リソースぶんの差分を求め、状態表を更新する。バリアが不要なら`None`を返す。
///
/// 省略規則: レイアウトが同一で、かつ前後どちらのアクセスにも書き込みが含まれない
/// （読み→読み）場合のみ省略する。それ以外(書き込みが絡む・レイアウトが変わる)は発行する。
fn 差分を計算する(
    現在状態: &mut HashMap<画像ハンドル, 画像状態>, ハンドル: 画像ハンドル, 今用途: 画像用途
) -> Option<画像バリア記述> {
    let 今状態 = 状態へ写像する(今用途);
    let 前状態 = *現在状態
        .get(&ハンドル)
        .unwrap_or_else(|| panic!("グラフに未登録の画像ハンドルがパスで使用された"));
    現在状態.insert(ハンドル, 今状態);

    let 省略可能 = 前状態.layout == 今状態.layout && !書き込みを含むか(前状態.access) && !書き込みを含むか(今状態.access);
    if 省略可能 {
        return None;
    }
    Some(画像バリア記述 {
        ハンドル,
        前: 前状態,
        今: 今状態,
    })
}
