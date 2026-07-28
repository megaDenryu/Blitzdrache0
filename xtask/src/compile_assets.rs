//! ソースアセットを版付き実行時形式へ変換する唯一の公開ツール入口。
//! 1つの出力ルートは1つのチャンク世界を持つため、世界ごとに出力ルートを分けて順に生成する。

use std::path::Path;
use std::process::{Command, ExitCode};

const 既定ソースルート: &str = "assets";
const 既定出力ルート: &str = "target/runtime_assets";
const 地形の既定出力ルート: &str = "target/terrain_assets";
const 植生の既定出力ルート: &str = "target/vegetation_assets";

/// プロセス境界で世界を指す名前。compile_assetsのexampleが同じ綴りを解析する。
pub const 板の世界: &str = "chunk_world";
pub const 地形の世界: &str = "terrain_world";
pub const 植生の世界: &str = "vegetation_world";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let 成否 = match 引数一覧 {
        [] => 既定を生成する() && 地形世界を既定で生成する() && 植生世界を既定で生成する(),
        [ソース, 出力] => 生成する(Path::new(ソース), Path::new(出力), 板の世界),
        [ソース, 出力, 世界名] => 生成する(Path::new(ソース), Path::new(出力), 世界名),
        _ => {
            eprintln!("使い方: cargo xtask compile-assets [ソースルート 出力ルート [世界名]]");
            return ExitCode::FAILURE;
        }
    };
    if 成否 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

pub fn 既定を生成する() -> bool {
    生成する(Path::new(既定ソースルート), Path::new(既定出力ルート), 板の世界)
}

/// 地形世界は板の世界と同じ座標を占めるため、同じ出力ルートへは同居できない。専用の既定出力ルートへ焼く。
pub fn 地形世界を既定で生成する() -> bool {
    生成する(Path::new(既定ソースルート), Path::new(地形の既定出力ルート), 地形の世界)
}

/// 植生世界も原点チャンクを占めるため、専用の既定出力ルートへ焼く。
pub fn 植生世界を既定で生成する() -> bool {
    生成する(Path::new(既定ソースルート), Path::new(植生の既定出力ルート), 植生の世界)
}

pub fn 生成する(ソースルート: &Path, 出力ルート: &Path, 世界名: &str) -> bool {
    println!(
        "[xtask] 実行時アセット生成({世界名}): {} -> {}",
        ソースルート.display(),
        出力ルート.display()
    );
    let 状態 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "compile_assets", "--"])
        .arg(ソースルート)
        .arg(出力ルート)
        .arg(世界名)
        .status();
    match 状態 {
        Ok(終了状態) if 終了状態.success() => true,
        Ok(終了状態) => {
            eprintln!("[xtask] 実行時アセット生成が終了コード{終了状態}で失敗");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}
