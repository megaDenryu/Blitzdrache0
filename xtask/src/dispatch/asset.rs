//! 資産生成・検査系コマンドの割り当て。command_catalogの`asset`分類と同じ範囲(glTF入力契約の
//! 検査・部品カタログ・検収描画・実行時アセット生成・検証用ソースアセット生成・生成物のソースアセットの取り揃えの15件)を担当する。

use std::process::ExitCode;

use crate::{
    check_glb, compile_assets, fetch_assets, gen_atmosphere_reference, gen_game_map, gen_source_assets, large_world_assets, part_row_draw,
    part_tools, watch_assets,
};

pub(super) fn アセットコマンドを割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "check-glb" => Some(check_glb::gltf入力契約を検査する(引数一覧)),
        "part-catalog" => Some(part_tools::カタログを組み上げる(引数一覧)),
        "part-assembly" => Some(part_tools::組み立てを突き合わせる(引数一覧)),
        "part-house-row-draw" => Some(part_row_draw::家の並びを検収する()),
        "part-frame-row-draw" => Some(part_row_draw::一間四方の骨格の並びを検収する()),
        "part-frame-chain-draw" => Some(part_row_draw::骨格を連ねた見本を撮影する()),
        "part-tree-row-draw" => Some(part_row_draw::木の並びを撮影する()),
        "compile-assets" => Some(compile_assets::実行時アセットを生成する(引数一覧)),
        "watch-assets" => Some(watch_assets::アセットを監視して再生成する(引数一覧)),
        "gen-source-assets" => Some(gen_source_assets::検証用ソースアセットを生成する()),
        "supply-source-assets" => Some(gen_source_assets::生成物のソースアセットを揃える()),
        "gen-atmosphere-reference" => Some(gen_atmosphere_reference::大気の期待値を生成する(引数一覧)),
        "gen-game-map" => Some(gen_game_map::クソゲー1本目の地図を生成する(引数一覧)),
        "large-world-assets" => Some(large_world_assets::大規模世界アセットを生成する(引数一覧)),
        "fetch-assets" => Some(fetch_assets::標準サンプルを取得する()),
        _ => None,
    }
}
