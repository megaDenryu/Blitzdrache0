//! コマンド行の引数を焼く指定へ写す工程。受け取るのは引数の並び、返すのは終了コードである。
//! 担当するのは、受け付ける引数の形と使い方の案内と、知らない世界名を告げることだけであり、
//! どの世界をどの出力ルートへ焼くかは親モジュールが持つ。

use std::path::Path;
use std::process::ExitCode;

use super::default_root::{地形世界を既定で生成する, 夜の多光源世界を既定で生成する, 既定を生成する, 植生世界を既定で生成する};
use super::{個体数を添えて生成する, 実行時形式を生成する};
use crate::asset_generator::{世界名, 同居植生の個体数};

pub fn 実行時アセットを生成する(引数一覧: &[String]) -> ExitCode {
    let 成否 = match 引数一覧 {
        [] => 既定を生成する() && 地形世界を既定で生成する() && 植生世界を既定で生成する() && 夜の多光源世界を既定で生成する(),
        [ソース, 出力] => 実行時形式を生成する(Path::new(ソース), Path::new(出力), 世界名::板の世界),
        [ソース, 出力, 文字列] => match 世界名::引数の文字列から解釈する(文字列) {
            Some(世界) => 実行時形式を生成する(Path::new(ソース), Path::new(出力), 世界),
            None => return 知らない世界名を告げる(文字列),
        },
        [ソース, 出力, 文字列, 個体数] => match (世界名::引数の文字列から解釈する(文字列), 個体数.parse::<usize>()) {
            (Some(世界), Ok(個体数)) => 個体数を添えて生成する(Path::new(ソース), Path::new(出力), 世界, Some(同居植生の個体数::生成する(個体数))),
            (None, _) => return 知らない世界名を告げる(文字列),
            (Some(_), Err(誤り)) => return 個体数を数として読めないことを告げる(個体数, &誤り),
        },
        _ => {
            eprintln!("使い方: cargo xtask compile-assets [ソースルート 出力ルート [世界名 [同居植生個体数]]]");
            return ExitCode::FAILURE;
        }
    };
    if 成否 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

/// 個体数を既定へ倒さず、読めなかった文字列を名指して落とす。既定へ倒すと、打ち間違えた実行が別の密度で焼かれる。
fn 個体数を数として読めないことを告げる(文字列: &str, 誤り: &std::num::ParseIntError) -> ExitCode {
    eprintln!("[xtask] 同居植生の個体数を数として読めない({文字列}): {誤り}");
    ExitCode::FAILURE
}

/// 知らない文字列を既定の世界へ倒さず、焼ける世界を並べて落とす。
fn 知らない世界名を告げる(文字列: &str) -> ExitCode {
    let 一覧: Vec<&str> = 世界名::一覧().iter().map(|世界| 世界.文字列()).collect();
    eprintln!("[xtask] 知らない世界名である: {文字列}(焼けるのは{})", 一覧.join("、"));
    ExitCode::FAILURE
}
