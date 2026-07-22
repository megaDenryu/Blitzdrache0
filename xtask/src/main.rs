//! 開発ツールの唯一の入口。`cargo xtask <コマンド>` で呼ぶ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」

use std::process::ExitCode;

mod bench;
mod compile_assets;
mod conform;
mod fetch_assets;
mod gen_smoke_asset;
mod m10_bench;
mod m11_soak;
mod object_bench;
mod smoke;
mod verify;
mod watch_assets;

fn main() -> ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match 引数一覧.first().map(String::as_str) {
        Some("verify") => verify::検証列を実行する(),
        Some("conform") => conform::実行する(),
        Some("smoke") => smoke::実行する(),
        Some("compile-assets") => compile_assets::実行する(&引数一覧[1..]),
        Some("watch-assets") => watch_assets::実行する(&引数一覧[1..]),
        Some("gen-smoke-asset") => gen_smoke_asset::実行する(),
        Some("fetch-assets") => fetch_assets::実行する(),
        Some("bench") => bench::実行する(),
        Some("m10-bench") => m10_bench::実行する(),
        Some("m11-soak") => m11_soak::実行する(),
        Some("object-bench") => object_bench::実行する(),
        _ => {
            使い方を表示する();
            ExitCode::FAILURE
        }
    }
}

fn 使い方を表示する() {
    println!("使い方: cargo xtask <コマンド>");
    println!();
    println!("コマンド一覧:");
    println!("  verify           検証の標準列 (conform -> check -> clippy -D warnings -> test) を実行する");
    println!("  conform          規約適合の機械検査 (100行制限/禁止文字列/不正allow/依存白リスト/参照パス実在)");
    println!("  smoke            blitz_appを--framesで自動実行し、validation件数0を終了コードで確認する");
    println!("  compile-assets   glTF・画像を検証し、target/runtime_assetsへ実行時形式を生成する");
    println!("  watch-assets     カタログのソース依存を監視し、変更時に実行時形式を再生成する");
    println!("  gen-smoke-asset  スモーク用極小glTFアセットをassets/smoke/へ生成する");
    println!("  fetch-assets     標準サンプル(DamagedHelmet・Fox)をassets/samples/へ取得する(curl.exe使用)");
    println!("  bench            リリース版の固定シーンを600フレーム実行し、GPU時間とCPU側フレーム間隔分布を表示する");
    println!("  m10-bench        M10流体GPU試作を固定条件で実行し、検証件数とGPU時間を表示する");
    println!("  m11-soak         3600フレーム連続実行し、RAM・VRAM推移を約5秒間隔で表示する");
    println!("  object-bench     二対象の画素判定後、1・10・100対象のGPU/CPU時間とGPUメモリを計測する");
}
