//! コマンド行の引数を焼く指定へ写す工程。受け取るのは引数の並び、返すのは終了コードである。
//! 担当するのは、受け付ける引数の形と使い方の案内と、知らない世界名を告げることだけであり、
//! どの世界をどの出力ルートへ焼くかは親モジュールが持つ。

use std::path::Path;
use std::process::ExitCode;

use super::default_root::{
    地形世界を既定で生成する, 夜の多光源世界を既定で生成する, 既定を生成する, 植生世界を既定で生成する
};
use super::world_name::世界名;
use super::{個体数を添えて生成する, 生成する};

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let 成否 = match 引数一覧 {
        [] => 既定を生成する() && 地形世界を既定で生成する() && 植生世界を既定で生成する() && 夜の多光源世界を既定で生成する(),
        [ソース, 出力] => 生成する(Path::new(ソース), Path::new(出力), 世界名::板の世界),
        [ソース, 出力, 綴り] => match 世界名::引数の綴りから解釈する(綴り) {
            Some(世界) => 生成する(Path::new(ソース), Path::new(出力), 世界),
            None => return 知らない世界名を告げる(綴り),
        },
        [ソース, 出力, 綴り, 個体数] => match 世界名::引数の綴りから解釈する(綴り) {
            Some(世界) => 個体数を添えて生成する(Path::new(ソース), Path::new(出力), 世界, Some(個体数)),
            None => return 知らない世界名を告げる(綴り),
        },
        _ => {
            eprintln!("使い方: cargo xtask compile-assets [ソースルート 出力ルート [世界名 [同居植生個体数]]]");
            return ExitCode::FAILURE;
        }
    };
    if 成否 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

/// 知らない綴りを既定の世界へ倒さず、焼ける世界を並べて落とす。
fn 知らない世界名を告げる(綴り: &str) -> ExitCode {
    let 一覧: Vec<&str> = 世界名::一覧().iter().map(|世界| 世界.綴り()).collect();
    eprintln!("[xtask] 知らない世界名である: {綴り}(焼けるのは{})", 一覧.join("、"));
    ExitCode::FAILURE
}
