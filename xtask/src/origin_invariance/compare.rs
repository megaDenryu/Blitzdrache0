//! 読み戻し画像2枚のバイト比較と結果表示。
//! 担当する工程: 画像の対と判定名を受け取り、一致しないときは差分画素数と最初の差分位置を添えた判定結果を返す。

use std::process::ExitCode;

use super::run::読み戻し画像;

pub(super) struct 判定結果 {
    名前: &'static str,
    詳細: Option<String>,
}

pub(super) fn 判定する(名前: &'static str, 左: &読み戻し画像, 右: &読み戻し画像) -> 判定結果 {
    if 左.寸法宣言 != 右.寸法宣言 {
        return 判定結果 {
            名前,
            詳細: Some(format!("寸法が違う: {} と {}", 左.寸法宣言.trim(), 右.寸法宣言.trim())),
        };
    }
    if 左.画素バイト列.len() != 右.画素バイト列.len() {
        return 判定結果 {
            名前,
            詳細: Some(format!("バイト長が違う: {} と {}", 左.画素バイト列.len(), 右.画素バイト列.len())),
        };
    }
    let 差分一覧: Vec<usize> = 左
        .画素バイト列
        .iter()
        .zip(右.画素バイト列.iter())
        .enumerate()
        .filter_map(|(添字, (左値, 右値))| (左値 != 右値).then_some(添字))
        .collect();
    let 詳細 = 差分一覧.first().map(|先頭| {
        format!(
            "差分バイト数{}件、最初の差分はバイト{}({}と{})",
            差分一覧.len(),
            先頭,
            左.画素バイト列[*先頭],
            右.画素バイト列[*先頭]
        )
    });
    判定結果 { 名前, 詳細 }
}

pub(super) fn 結果を表示する(判定一覧: &[判定結果]) -> ExitCode {
    let mut 全て一致 = true;
    for 判定 in 判定一覧 {
        match &判定.詳細 {
            None => println!("[xtask] 一致: {}", 判定.名前),
            Some(詳細) => {
                eprintln!("[xtask] 不一致: {} — {詳細}", 判定.名前);
                全て一致 = false;
            }
        }
    }
    if 全て一致 {
        println!("[xtask] origin-invariance成功: 原点移動に対して読み戻し画像がバイト一致した");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
