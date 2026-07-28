//! 開発ツールの唯一の入口。`cargo xtask <コマンド>` で呼ぶ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」

use std::process::ExitCode;

mod bench;
mod compile_assets;
mod conform;
mod fetch_assets;
mod file_scan;
mod gen_source_assets;
mod instance_cull;
mod instance_draw;
mod lod_crack;
mod m10_bench;
mod m11_soak;
mod memory_sampling;
mod object_bench;
mod origin_invariance;
mod ow3_dod;
mod pixel_region;
mod release_build;
mod report_parse;
mod shader_copy;
mod smoke;
mod streaming_bench;
mod type_metrics;
mod vegetation_run;
mod verify;
mod watch_assets;

fn main() -> ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match 引数一覧.first().map(String::as_str) {
        Some("verify") => verify::検証列を実行する(),
        Some("conform") => conform::実行する(),
        Some("type-metrics") => type_metrics::実行する(),
        Some("smoke") => smoke::実行する(),
        Some("compile-assets") => compile_assets::実行する(&引数一覧[1..]),
        Some("watch-assets") => watch_assets::実行する(&引数一覧[1..]),
        Some("gen-source-assets") => gen_source_assets::実行する(),
        Some("fetch-assets") => fetch_assets::実行する(),
        Some("bench") => bench::実行する(),
        Some("bench-display-timing") => bench::実表示計測つきで実行する(),
        Some("m10-bench") => m10_bench::実行する(),
        Some("m11-soak") => m11_soak::実行する(),
        Some("object-bench") => object_bench::実行する(),
        Some("origin-invariance") => origin_invariance::実行する(),
        Some("lod-crack") => lod_crack::実行する(),
        Some("instance-draw") => instance_draw::実行する(),
        Some("instance-cull") => instance_cull::実行する(),
        Some("ow3-dod") => ow3_dod::実行する(),
        Some("streaming-bench") => streaming_bench::実行する(&引数一覧[1..]),
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
    println!("  verify           検証の標準列 (conform -> fmt --check -> check -> clippy -D warnings -> test) を実行する");
    println!("  conform          規約適合の機械検査 (100行制限/禁止文字列/不正allow/依存白リスト/参照パス実在/節参照実在/vulkan配下のDrop実装禁止)");
    println!("  type-metrics     型ごとのフィールド数・impl分散ファイル数・メソッド数を多い順に表示する (違反判定はしない)");
    println!("  smoke            blitz_appを--framesで自動実行し、validation件数0を終了コードで確認する");
    println!(
        "  compile-assets [ソースルート 出力ルート [世界名]]  ソースを検証して実行時形式を生成する(引数なしでchunk_worldをtarget/runtime_assetsへ、terrain_worldをtarget/terrain_assetsへ)"
    );
    println!("  watch-assets     カタログのソース依存を監視し、変更時に実行時形式を再生成する");
    println!(
        "  gen-source-assets 検証用ソースアセット(スモーク用quad・影検証シーン・板の世界25チャンク・地形世界25チャンクの高さ格子)をassets/へ生成する"
    );
    println!("  fetch-assets     標準サンプル(DamagedHelmet・Fox)をassets/samples/へ取得する(curl.exe使用)");
    println!("  bench            リリース版の固定シーンを600フレーム実行し、GPU時間とCPU側フレーム間隔分布を表示する");
    println!("  bench-display-timing  benchに実表示間隔の計測を足して実行する(計測が描画ループを止めるため既存の時系列とは比較できない)");
    println!("  m10-bench        M10流体GPU試作を固定条件で実行し、検証件数とGPU時間を表示する");
    println!("  m11-soak         3600フレーム連続実行し、RAM・VRAM推移を約5秒間隔で表示する");
    println!("  object-bench     二対象の画素判定後、1・10・100対象のGPU/CPU時間とGPUメモリを計測する");
    println!("  origin-invariance 世界全体へkm級の大域平行移動を加えても読み戻し画像がバイト一致し、カメラだけを微小に動かすと変わることを確かめる");
    println!("  lod-crack        地形LODの段差0・1・最大を四方向と細粗入替でGPU描画し、内側の継ぎ目に背景色が露出しないことを確かめる");
    println!(
        "  instance-draw    植生インスタンス群を実機描画し、4個体が画面の4分割へ離れて描かれること・両パスの発行が群×段ごと1回であること・個体数を増やしてもGPU確保数が増えないことを確かめる"
    );
    println!(
        "  instance-cull    可視判定のオンとオフで植生シーンを描き、画面内の絵がバイト一致すること・視錐台外の個体が描かれなくなること・画面外の個体が落とす影が残ることを確かめる"
    );
    println!("  ow3-dod          原点移動・LOD継ぎ目・半径2ストリーミングを本番経路でまとめて測り、複数LOD画像をPNGへ書き出す");
    println!(
        "  streaming-bench [フレーム数]  固定経路でチャンクを読み込みながら、予算を十分に取った反復のRAM・VRAM推移と、縮退させた読込・解除順の再現を測る"
    );
}
